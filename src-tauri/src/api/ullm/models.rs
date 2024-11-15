use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::parameters::EngineParameters;

#[derive(Debug, Deserialize, Default)]
pub struct Response<T> {
    pub id: Uuid,
    pub result: T,
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
pub enum CompletionStatus {
    #[serde(rename = "success")]
    #[default]
    Success,
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "final")]
    Final,
}

impl Display for CompletionStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CompletionStatus::Success => write!(f, "Success"),
            CompletionStatus::Pong => write!(f, "Pong"),
            CompletionStatus::Ongoing => write!(f, "Ongoing"),
            CompletionStatus::Final => write!(f, "Final"),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct CompletionResult {
    pub status: CompletionStatus,
    pub tokens: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct ListResult {
    pub models: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct SimpleResult {
    pub status: CompletionStatus,
}

#[derive(Debug, Serialize)]
pub struct LoadParams {
    pub engine: String,
    pub model: String,
}

#[derive(Debug, Serialize)]
pub struct CompletionParams {
    pub snippet: String,
    pub engine_parameters: EngineParameters,
}