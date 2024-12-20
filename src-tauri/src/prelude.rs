use thiserror::Error;

pub type Result<T> = std::result::Result<T, AliceError>;

#[derive(Debug, Error)]
pub enum AliceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),

    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    // Serde
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    // Tauri
    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    // OnceLock
    #[error("OnceLock is empty")]
    OnceLockEmpty,

    // SurrealDB
    #[error("SurrealDB error: {0}")]
    SurrealDB(#[from] surrealdb::Error),
    #[error("Failed to `{0}` SurrealDB Object with record: {1}")]
    DatabaseOperation(String, String),

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

impl From<AliceError> for String {
    fn from(value: AliceError) -> Self {
        format!("{}", value)
    }
}

impl From<&str> for AliceError {
    fn from(value: &str) -> Self {
        AliceError::Other(value.to_string())
    }
}
