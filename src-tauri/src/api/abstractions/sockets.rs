use crate::prelude::*;
use futures_util::{Future, SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct ClientSocket {
    addr: String,
    stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl ClientSocket {
    pub fn new(addr: String) -> Result<Self> {
        Ok(Self { addr, stream: None })
    }

    pub async fn connect(&mut self) -> Result<()> {
        let (stream, _) = tokio_tungstenite::connect_async(&self.addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(AliceError::NoStream);
        };
        stream.close(None).await?;
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        self.stream.is_some()
    }

    pub async fn send_str(&mut self, str: String) -> Result<()> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(AliceError::NoStream);
        };
        Ok(stream.send(Message::Text(str)).await?)
    }

    pub async fn send_bin(&mut self, bin: Vec<u8>) -> Result<()> {
        let Some(stream) = self.stream.as_mut() else {
            return Err(AliceError::NoStream);
        };
        Ok(stream.send(Message::Binary(bin)).await?)
    }

    pub async fn return_single<T: DeserializeOwned + Default>(&mut self) -> Result<T> {
        self.return_streaming(|_| Ok(true), |_| async { Ok(()) })
            .await
    }

    pub async fn return_streaming<T, F, G, Fut>(
        &mut self,
        should_stop: F,
        streaming_callback: G,
    ) -> Result<T>
    where
        T: DeserializeOwned + Default,
        F: Fn(&T) -> Result<bool>,
        Fut: Future<Output = Result<()>>,
        G: Fn(T) -> Fut,
    {
        let Some(stream) = self.stream.as_mut() else {
            return Err(AliceError::NoStream);
        };
        while let Some(response) = stream.next().await {
            let Ok(response) = response else {
                return Err(AliceError::ResponseError);
            };
            match response {
                Message::Text(json) => {
                    let result: T = serde_json::from_str(&json)?;
                    if should_stop(&result)? {
                        return Ok(result);
                    }
                    streaming_callback(result).await?;
                }
                Message::Binary(_bin) => {
                    todo!()
                }
                Message::Ping(ping) => {
                    stream.send(Message::Pong(ping)).await?;
                }
                Message::Close(_) => {
                    self.stream = None;
                    break;
                }
                _ => {
                    return Err(AliceError::InvalidMessage);
                }
            }
        }
        Ok(T::default())
    }
}
