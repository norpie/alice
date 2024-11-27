use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{model::{Engine, Model}, parameters::EngineParameters};

#[derive(Debug, Deserialize, Default)]
pub struct Response<T> {
    pub id: Uuid,
    pub result: T,
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
pub enum ModelStatus {
    #[serde(rename = "loading")]
    Loading,
    #[serde(rename = "loaded")]
    Loaded,
    #[default]
    #[serde(rename = "unloaded")]
    Unloaded,
    #[serde(rename = "error")]
    Error,
}

impl Display for ModelStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ModelStatus::Loading => write!(f, "loading"),
            ModelStatus::Loaded => write!(f, "loaded"),
            ModelStatus::Unloaded => write!(f, "unloaded"),
            ModelStatus::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq)]
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

#[derive(Default, Debug, Deserialize)]
pub struct StatusResult {
    pub status: String,
    pub engine: Option<Engine>,
    pub model: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct ModelListResult {
    pub models: Vec<Model>,
}
