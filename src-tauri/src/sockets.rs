use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{calls::MethodCall, responses::Response};

static API_URL: &str = "ws://localhost:8081";

pub struct SocketManager {
    pub stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl SocketManager {
    pub async fn connect() -> SocketManager {
        let (stream, _) = tokio_tungstenite::connect_async(API_URL)
            .await
            .expect("Failed to connect");
        SocketManager { stream }
    }

    pub async fn close(&mut self) {
        self.stream
            .close(None)
            .await
            .expect("Failed to close connection");
    }

    pub async fn call<T: Serialize, U: DeserializeOwned>(
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
}
