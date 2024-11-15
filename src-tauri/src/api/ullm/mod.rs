use async_trait::async_trait;
use models::{CompletionParams, CompletionResult, CompletionStatus, LoadParams, Response};
use uuid::Uuid;

use crate::{
    models::{model::Model, parameters::EngineParameters},
    prelude::*,
};

use super::{
    abstractions::{sockets::ClientSocket, MethodCall},
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

    async fn keep_alive(&mut self) -> Result<()> {
        self.client.return_single().await
    }

    async fn load(&mut self, model: &Model) -> Result<()> {
        let load = MethodCall {
            id: Uuid::new_v4(),
            method: "load".to_string(),
            params: Some(LoadParams {
                engine: model.engine.to_string(),
                model: model.name.clone(),
            }),
        };
        self.client.send_str(serde_json::to_string(&load)?).await
    }

    async fn unload(&mut self) -> Result<()> {
        let unload: MethodCall<()> = MethodCall {
            id: Uuid::new_v4(),
            method: "unload".to_string(),
            params: None,
        };
        self.client.send_str(serde_json::to_string(&unload)?).await
    }

    async fn list(&mut self) -> Result<Vec<Model>> {
        let list: MethodCall<()> = MethodCall {
            id: Uuid::new_v4(),
            method: "list".to_string(),
            params: None,
        };
        self.client.send_str(serde_json::to_string(&list)?).await?;
        self.client.return_single().await
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
