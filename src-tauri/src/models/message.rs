use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub role: String,
    pub content: String,
}
