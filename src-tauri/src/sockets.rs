use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    calls::{CompletionParams, LoadParams, MethodCall},
    models::parameters::EngineParameters,
    responses::{CompletionResult, ListResult, Response, SimpleResult, CompletionStatus},
};

#[derive(Debug, Eq, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
}

pub struct UllmAPI {
    pub address: String,
    pub stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub status: ConnectionStatus,
    pub last_ping: Option<std::time::Instant>,
}

impl UllmAPI {
    pub async fn new(url: &str) -> Self {
        UllmAPI {
            address: url.to_string(),
            stream: None,
            status: ConnectionStatus::Disconnected,
            last_ping: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let (stream, _) = tokio_tungstenite::connect_async(&self.address)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        self.stream = Some(stream);
        self.status = ConnectionStatus::Connected;
        self.last_ping = Some(std::time::Instant::now());
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.stream
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stream to close"))?
            .close(None)
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        self.last_ping = None;
        self.status = ConnectionStatus::Disconnected;
        Ok(())
    }

    async fn call_single<T: Serialize, U: DeserializeOwned>(
        &mut self,
        call: MethodCall<T>,
    ) -> Result<Response<U>> {
        // Convert the message to a JSON string
        let json_msg = serde_json::to_string(&call).map_err(|e| anyhow::anyhow!(e))?;

        // Send the JSON message over the WebSocket
        self.stream
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stream to use"))?
            .send(Message::Text(json_msg))
            .await
            .expect("Failed to send message");
        loop {
            if let Some(response) = self.stream.as_mut().unwrap().next().await {
                // Wait for the response
                dbg!(&response);
                match response {
                    Ok(msg) => match msg {
                        Message::Text(txt) => {
                            // Parse the JSON response
                            self.last_ping = Some(std::time::Instant::now());
                            return serde_json::from_str(&txt).map_err(|e| anyhow::anyhow!(e));
                        }
                        Message::Close(close) => {
                            anyhow::bail!("Connection closed: {:?}", close);
                        }
                        Message::Ping(ping) => {
                            let res = self
                                .stream
                                .as_mut()
                                .unwrap()
                                .send(Message::Pong(ping))
                                .await;
                            res.map_err(|e| anyhow::anyhow!(e))?;
                        }
                        _ => anyhow::bail!("Unexpected message type, {:?}", msg),
                    },
                    Err(e) => anyhow::bail!(e),
                }
            }
        }
    }

    pub async fn is_alive(&mut self) -> bool {
        let call: MethodCall<()> = MethodCall {
            method: "ping".to_string(),
            id: uuid::Uuid::new_v4(),
            params: None,
        };
        let response: Result<Response<SimpleResult>> = self.call_single(call).await;
        response.is_ok()
    }

    pub async fn ping(&mut self) -> Result<Response<SimpleResult>> {
        let call: MethodCall<()> = MethodCall {
            method: "ping".to_string(),
            id: uuid::Uuid::new_v4(),
            params: None,
        };
        self.call_single(call).await
    }

    pub async fn list_models(&mut self) -> Result<Response<ListResult>> {
        let call: MethodCall<()> = MethodCall {
            method: "list_models".to_string(),
            id: uuid::Uuid::new_v4(),
            params: None,
        };
        self.call_single(call).await
    }

    pub async fn load_model(
        &mut self,
        engine: &str,
        model: &str,
    ) -> Result<Response<SimpleResult>> {
        let call: MethodCall<LoadParams> = MethodCall {
            method: "load_model".to_string(),
            id: uuid::Uuid::new_v4(),
            params: Some(LoadParams {
                engine: engine.to_string(),
                model: model.to_string(),
            }),
        };
        self.call_single(call).await
    }

    pub async fn completion<Fut: std::future::Future<Output = ()>, Fun: Fn(String) -> Fut>(
        &mut self,
        snippet: String,
        callback: Fun,
    ) -> Result<String> {
        let mut parameters = EngineParameters::default();
        parameters.stop_sequences.push("user:".to_string());
        let call = MethodCall {
            id: uuid::Uuid::new_v4(),
            method: "complete".to_string(),
            params: Some(CompletionParams {
                snippet,
                engine_parameters: parameters,
            }),
        };

        // Convert the message to a JSON string
        let json_msg = serde_json::to_string(&call).map_err(|e| anyhow::anyhow!(e))?;

        // Send the JSON message over the WebSocket
        self.stream
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stream to use"))?
            .send(Message::Text(json_msg))
            .await
            .expect("Failed to send message");

        while let Some(response) = self.stream.as_mut().unwrap().next().await {
            // Wait for the response
            match response {
                Ok(msg) => match msg {
                    Message::Text(txt) => {
                        // Parse the JSON response
                        self.last_ping = Some(std::time::Instant::now());
                        let response: Response<CompletionResult> =
                            serde_json::from_str(&txt).map_err(|e| anyhow::anyhow!(e))?;
                        match response.result.status {
                            CompletionStatus::Ongoing => {
                                callback(response.result.tokens).await;
                            }
                            CompletionStatus::Final => {
                                return Ok(response.result.tokens);
                            }
                            _ => anyhow::bail!("Unexpected status"),
                        }
                    }
                    Message::Close(close) => {
                        anyhow::bail!("Connection closed: {:?}", close);
                    }
                    Message::Ping(ping) => {
                        self.stream
                            .as_mut()
                            .unwrap()
                            .send(Message::Pong(ping))
                            .await?;
                    }
                    _ => anyhow::bail!("Unexpected message type, {:?}", msg),
                },
                Err(e) => anyhow::bail!(e),
            }
        }
        anyhow::bail!("No response received")
    }
}
