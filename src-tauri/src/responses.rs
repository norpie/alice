use std::fmt::{self, Display, Formatter};

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

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Pong => write!(f, "Pong"),
            Status::Ongoing => write!(f, "Ongoing"),
            Status::Final => write!(f, "Final"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompletionResult {
    pub status: Status,
    pub tokens: String,
}

#[derive(Debug, Deserialize)]
pub struct ListResult {
    pub models: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleResult {
    pub status: Status,
}
