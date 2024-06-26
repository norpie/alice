use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{
    calls::{CompletionParams, LoadParams, MethodCall},
    responses::{CompletionResult, ListResult, Response, SimpleResult, Status},
};

pub struct UllmAPI {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl UllmAPI {
    pub async fn new(url: &str) -> Result<Self> {
        let (stream, _) = tokio_tungstenite::connect_async(url)
            .await
            .expect("Failed to connect");
        Ok(UllmAPI { stream })
    }

    pub async fn close(&mut self) {
        self.stream
            .close(None)
            .await
            .expect("Failed to close connection");
    }

    async fn call_single<T: Serialize, U: DeserializeOwned>(
        &mut self,
        call: MethodCall<T>,
    ) -> Result<Response<U>> {
        // Convert the message to a JSON string
        let json_msg = serde_json::to_string(&call).map_err(|e| anyhow::anyhow!(e))?;

        // Send the JSON message over the WebSocket
        self.stream
            .send(Message::Text(json_msg))
            .await
            .expect("Failed to send message");

        if let Some(response) = self.stream.next().await {
            // Wait for the response
            match response {
                Ok(msg) => match msg {
                    Message::Text(txt) => {
                        // Parse the JSON response
                        println!("{}", txt);
                        Ok(serde_json::from_str(&txt).unwrap())
                    }
                    _ => anyhow::bail!("Unexpected message type"),
                },
                Err(e) => anyhow::bail!(e),
            }
        } else {
            anyhow::bail!("No response received")
        }
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

    pub async fn load_model(&mut self, engine: &str, model: &str) -> Result<Response<SimpleResult>> {
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
        let call = MethodCall {
            id: uuid::Uuid::new_v4(),
            method: "complete".to_string(),
            params: Some(CompletionParams { snippet }),
        };

        // Convert the message to a JSON string
        let json_msg = serde_json::to_string(&call).map_err(|e| anyhow::anyhow!(e))?;

        // Send the JSON message over the WebSocket
        self.stream
            .send(Message::Text(json_msg))
            .await
            .expect("Failed to send message");

        while let Some(response) = self.stream.next().await {
            // Wait for the response
            match response {
                Ok(msg) => match msg {
                    Message::Text(txt) => {
                        // Parse the JSON response
                        let response: Response<CompletionResult> =
                            serde_json::from_str(&txt).map_err(|e| anyhow::anyhow!(e))?;
                        match response.result.status {
                            Status::Ongoing => {
                                callback(response.result.tokens).await;
                            }
                            Status::Final => {
                                return Ok(response.result.tokens);
                            }
                            _ => anyhow::bail!("Unexpected status"),
                        }
                    }
                    _ => anyhow::bail!("Unexpected message type"),
                },
                Err(e) => anyhow::bail!(e),
            }
        }
        anyhow::bail!("No response received")
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::{
        calls::{LoadParams, MethodCall},
        responses::Response,
    };

    #[tokio::test]
    async fn test_complete() {
        let mut api = super::UllmAPI::new("ws://localhost:8081").await.unwrap();
        let list_call: MethodCall<()> = super::MethodCall {
            method: "list_models".to_string(),
            id: uuid::Uuid::new_v4(),
            params: None,
        };
        let response: Response<Value> = api.call_single(list_call).await.unwrap();
        println!("{:?}", response);
        let load_call = super::MethodCall {
            method: "load_model".to_string(),
            id: uuid::Uuid::new_v4(),
            params: Some(LoadParams {
                engine: "exllamav2".into(),
                model: "dolphin-2.9.1-llama-3-8b".into(),
            }),
        };
        let response: Response<Value> = api.call_single(load_call).await.unwrap();
        println!("{:?}", response);
        let snippet = "fn main() { let x = 1; x.".to_string();
        print!("Snippet: {}", snippet);
        let completion = api
            .completion(snippet, |tokens| async move {
                print!("{}", tokens);
            })
            .await
            .unwrap();
        println!("Full completion: {}", completion);
    }
}
