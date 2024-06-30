use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
    pub messages: Vec<Message>,
}

impl History {
    pub fn into_partial(self, index: usize) -> PartialHistory {
        let messages = self.messages.into_iter().skip(index).collect();
        PartialHistory {
            id: self.id,
            initial_index: index,
            messages,
        }
    }

    // Remove messages from after the partial.initial_index, and append the partial.messages
    pub fn merge_partial(&mut self, partial: PartialHistory) {
        self.messages.truncate(partial.initial_index);
        self.messages.extend(partial.messages);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialHistory {
    pub id: Uuid,
    #[serde(rename = "initialIndex")]
    pub initial_index: usize,
    pub messages: Vec<Message>,
}

impl PartialHistory {
    pub fn into_history(self) -> History {
        History {
            timestamp: Utc::now(),
            id: self.id,
            messages: self.messages,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_partial() {
        let history = History {
            timestamp: Utc::now(),
            id: Uuid::new_v4(),
            messages: vec![
                Message {
                    timestamp: Utc::now(),
                    author: "Alice".to_string(),
                    content: "Hello".to_string(),
                },
                Message {
                    timestamp: Utc::now(),
                    author: "Bob".to_string(),
                    content: "Hi, how are you?".to_string(),
                },
            ],
        };
        let partial = history.into_partial(1);
        assert_eq!(partial.messages.len(), 1);
        assert_eq!(partial.messages[0].author, "Bob");
    }

    #[test]
    fn test_merge_partial() {
        let mut history = History {
            timestamp: Utc::now(),
            id: Uuid::new_v4(),
            messages: vec![
                Message {
                    timestamp: Utc::now(),
                    author: "Alice".to_string(),
                    content: "Hello".to_string(),
                },
                Message {
                    timestamp: Utc::now(),
                    author: "Bob".to_string(),
                    content: "Hi, how are you?".to_string(),
                },
            ],
        };
        let partial = PartialHistory {
            id: history.id,
            initial_index: 1,
            messages: vec![
                Message {
                    timestamp: Utc::now(),
                    author: "Bob".to_string(),
                    content: "Hi, how are you?".to_string(),
                },
                Message {
                    timestamp: Utc::now(),
                    author: "Alice".to_string(),
                    content: "I'm good, thanks!".to_string(),
                },
            ],
        };
        history.merge_partial(partial);
        assert_eq!(history.messages.len(), 3);
        assert_eq!(history.messages[0].author, "Alice");
        assert_eq!(history.messages[1].author, "Bob");
        assert_eq!(history.messages[2].author, "Alice");
    }
}
