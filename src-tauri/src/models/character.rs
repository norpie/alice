use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    id: Uuid,
    name: String,
    description: Option<String>,
    attributes: Vec<Attribute>,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            id: Uuid::new_v4(),
            name: "Nika Orchid".to_string(),
            description: Some("Nika calls you Master".to_string()),
            attributes: vec![],
        }
    }
}

impl Character {
    pub fn new(name: &str) -> Self {
        Character {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: None,
            attributes: vec![],
        }
    }

    pub fn add_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }
}
