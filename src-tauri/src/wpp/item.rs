use serde::{Deserialize, Serialize};

use super::{parser, format};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    values: Vec<String>,
}

impl Attribute {
    pub fn new(name: &str) -> Self {
        Attribute {
            name: name.to_string(),
            values: vec![],
        }
    }

    pub fn add_value(&mut self, value: &str) {
        self.values.push(value.to_string());
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WppItem {
    item_type: String,
    name: String,
    attributes: Vec<Attribute>,
}

impl WppItem {
    pub fn new(item_type: &str, name: &str) -> Self {
        Self {
            item_type: item_type.to_string(),
            name: name.to_string(),
            attributes: Vec::new(),
        }
    }

    pub fn item_type(&self) -> &str {
        &self.item_type
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn add_attribute(&mut self, attribute: Attribute) {
        self.attributes.push(attribute);
    }

    pub fn prompt(&self) -> String {
        format::format(self)
    }
}

impl TryFrom<&str> for WppItem {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        parser::parse(input).map_err(|e| anyhow::anyhow!(e))
    }
}
