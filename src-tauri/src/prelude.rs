use thiserror::Error;

pub type Result<T> = std::result::Result<T, AliceError>;

#[derive(Debug, Error)]
pub enum AliceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    // Tauri
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    // OnceLock
    #[error("OnceLock is empty")]
    OnceLockEmpty,

    // Handlebars
    #[error("Handlebars error: {0}")]
    Handlebars(#[from] handlebars::RenderError),

    // Sockets
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("No stream")]
    NoStream,
    #[error("Response error")]
    ResponseError,
    #[error("Invalid message")]
    InvalidMessage,
}
