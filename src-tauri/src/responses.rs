use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub id: Uuid,
    pub result: T,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "final")]
    Final,
}

#[derive(Debug, Deserialize)]
pub struct CompletionResult {
    pub status: Status,
    pub tokens: String,
}

#[derive(Debug, Deserialize)]
pub struct SimpleResult {
    pub status: Status,
}
