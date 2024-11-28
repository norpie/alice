use crate::prelude::*;

use crate::DB;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::opt::PatchOp;
use surrealdb::RecordId;

use crate::models::message::Message;

#[derive(Debug, Serialize, Deserialize)]
pub struct LeanConversation {
    pub id: RecordId,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertableConversation {
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
    pub messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: RecordId,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub modified_time: DateTime<Utc>,
    pub messages: Vec<Message>,
}

impl Default for InsertableConversation {
    fn default() -> Self {
        let time = Utc::now();
        Self {
            name: format!("Unnamed Conversation - {}", time.to_rfc3339()),
            start_time: time,
            modified_time: time,
            messages: Vec::new(),
        }
    }
}

impl Conversation {
    pub async fn new() -> Result<Self> {
        db!()
            .create("conversation")
            .content(InsertableConversation::default())
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "create".into(),
                "conversation".into(),
            ))
    }

    pub async fn find(id: String) -> Result<Self> {
        db!()
            .select(("conversation", &id))
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "select".into(),
                id.to_string(),
            ))
    }

    pub async fn date_sorted_lean(limit: usize, offset: usize) -> Result<Vec<LeanConversation>> {
        let mut result = db!()
            .query(
                "SELECT id, name, start_time, modified_time FROM conversation ORDER BY modified_time DESC LIMIT $limit START $offset",
            )
            .bind(("limit", limit))
            .bind(("offset", offset))
            .await?.take(0)?;
        dbg!(&result);
        Ok(result)
    }

    pub async fn with_name(self, name: String) -> Result<Self> {
        let db = db!();
        db.update(self.id)
            .patch(PatchOp::add("/name", name))
            .patch(PatchOp::replace("/modified_time", Utc::now()))
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "update".into(),
                "conversation".into(),
            ))
    }

    pub async fn without_message(self, index: usize) -> Result<Self> {
        let db = db!();
        db.update(self.id)
            .patch(PatchOp::remove(&format!("/messages/{}", index)))
            .patch(PatchOp::replace("/modified_time", Utc::now()))
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "update".into(),
                "conversation".into(),
            ))
    }

    pub async fn with_replaced_message(self, index: usize, content: String) -> Result<Self> {
        let db = db!();
        db.update(self.id)
            .patch(PatchOp::replace(
                &format!("/messages/{}", index),
                Message {
                    timestamp: Utc::now(),
                    role: self
                        .messages
                        .get(index)
                        .ok_or(AliceError::IndexOutOfBounds(index))?
                        .role
                        .clone(),
                    content,
                },
            ))
            .patch(PatchOp::replace("/modified_time", Utc::now()))
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "update".into(),
                "conversation".into(),
            ))
    }

    pub async fn with_message(self, role: String, content: String) -> Result<Self> {
        let db = db!();
        db.update(self.id)
            .patch(PatchOp::add(
                "/messages/-",
                Message {
                    timestamp: Utc::now(),
                    role,
                    content,
                },
            ))
            .patch(PatchOp::replace("/modified_time", Utc::now()))
            .await?
            .ok_or(AliceError::DatabaseOperation(
                "update".into(),
                "conversation".into(),
            ))
    }
}
