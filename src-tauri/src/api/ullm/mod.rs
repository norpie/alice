use std::collections::HashMap;

use async_trait::async_trait;
use models::{
    CompletionParams, CompletionResult, CompletionStatus, LoadParams, ModelListResult, ModelStatus,
    Response, StatusResult,
};
use serde_json::{Map, Value};
use uuid::Uuid;

use crate::{
    models::{model::Model, parameters::EngineParameters},
    prelude::*,
};

use super::{
    abstractions::{sockets::ClientSocket, MethodCall, MethodReturn},
    Api,
};

mod models;

pub struct UllmApi {
    client: ClientSocket,
}

impl UllmApi {
    pub fn new(addr: String) -> Result<Self> {
        Ok(Self {
            client: ClientSocket::new(addr)?,
        })
    }
}

#[async_trait]
impl Api for UllmApi {
    async fn connect(&mut self) -> Result<()> {
        self.client.connect().await
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.client.disconnect().await
    }

    async fn is_alive(&mut self) -> Result<bool> {
        let result = self.client.send_ping().await;
        if result.is_err() {
            let _ = self.client.disconnect().await;
            return Ok(false);
        }
        let _ = self.client.receive_pong().await?;
        Ok(true)
    }

    async fn load(
        &mut self,
        model: &Model,
        preload_callback: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
    ) -> Result<String> {
        fn should_stop(response: &Response<StatusResult>) -> Result<bool> {
            Ok(response.result.status == "loaded" || response.result.status == "error")
        }

        let load = MethodCall {
            id: Uuid::new_v4(),
            method: "load_model".to_string(),
            params: Some(LoadParams {
                engine: model.engine.to_string(),
                model: model.name.clone(),
            }),
        };

        self.client.send_str(serde_json::to_string(&load)?).await?;

        Ok(self
            .client
            .return_streaming(should_stop, |response| async {
                preload_callback(response.result.status)
            })
            .await?
            .result
            .status
            .to_string())
    }

    async fn unload(&mut self) -> Result<()> {
        let unload: MethodCall<()> = MethodCall {
            id: Uuid::new_v4(),
            method: "unload".to_string(),
            params: None,
        };

        self.client
            .send_str(serde_json::to_string(&unload)?)
            .await?;

        self.client
            .return_single::<MethodReturn<StatusResult>>()
            .await
            .map(|_| ())
    }

    async fn current(&mut self) -> Result<Option<Model>> {
        let status: MethodCall<()> = MethodCall {
            id: Uuid::new_v4(),
            method: "status".to_string(),
            params: None,
        };

        self.client
            .send_str(serde_json::to_string(&status)?)
            .await?;

        self.client
            .return_single::<Response<StatusResult>>()
            .await
            .map(|response| {
                response
                    .result
                    .model
                    .zip(response.result.engine)
                    .map(|(model, engine)| Model {
                        name: model,
                        engine,
                    })
            })
    }

    async fn list(&mut self) -> Result<Vec<Model>> {
        let list: MethodCall<()> = MethodCall {
            id: Uuid::new_v4(),
            method: "list_models".to_string(),
            params: None,
        };

        self.client.send_str(serde_json::to_string(&list)?).await?;

        Ok(self
            .client
            .return_single::<Response<ModelListResult>>()
            .await?
            .result
            .models)
    }

    async fn complete(
        &mut self,
        snippet: &str,
        engine_parameters: EngineParameters,
        streaming_callback: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
    ) -> Result<String> {
        fn should_stop(response: &Response<CompletionResult>) -> Result<bool> {
            Ok(response.result.status == CompletionStatus::Final)
        }

        let complete = MethodCall {
            id: Uuid::new_v4(),
            method: "complete".to_string(),
            params: Some(CompletionParams {
                snippet: snippet.to_string(),
                engine_parameters,
            }),
        };

        self.client
            .send_str(serde_json::to_string(&complete)?)
            .await?;

        Ok(self
            .client
            .return_streaming(should_stop, |response| async {
                streaming_callback(response.result.tokens)
            })
            .await?
            .result
            .tokens)
    }
}
