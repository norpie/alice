use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod sockets;

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodCall<T> {
    pub id: Uuid,
    pub method: String,
    pub params: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MethodReturn<T> {
    pub id: Uuid,
    pub result: Option<T>,
    pub error: Option<String>,
}

impl<T> Default for MethodReturn<T> {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            result: None,
            error: None,
        }
    }
}
