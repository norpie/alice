use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MethodCall<T> {
    pub id: Uuid,
    pub method: String,
    pub params: Option<T>,
}

#[derive(Debug, Serialize)]
pub struct LoadParams {
    pub engine: String,
    pub model: String
}

#[derive(Debug, Serialize)]
pub struct CompletionParams {
    pub snippet: String,
}
