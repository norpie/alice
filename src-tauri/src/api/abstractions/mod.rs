use serde::Serialize;
use uuid::Uuid;

pub mod sockets;

#[derive(Debug, Serialize)]
pub struct MethodCall<T> {
    pub id: Uuid,
    pub method: String,
    pub params: Option<T>,
}
