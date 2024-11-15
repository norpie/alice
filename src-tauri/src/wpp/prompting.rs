use std::collections::HashMap;

use crate::{models::message::Message, prelude::*};

use serde_json::Value;

pub struct Prompt {
    pub template: String,
    pub vars: HashMap<String, Value>,
}

impl Prompt {
    pub fn new(template: String) -> Self {
        Self {
            template,
            vars: HashMap::new(),
        }
    }

    pub fn render(&self) -> Result<String> {
        Ok(handlebars::Handlebars::new().render_template(&self.template, &self.vars)?)
    }

    pub fn with_str_var(mut self, var: &str, value: &str) -> Self {
        self.vars.insert(
            var.to_string(),
            serde_json::Value::String(value.to_string()),
        );
        self
    }

    pub fn with_messages(mut self, messages: Vec<Message>) -> Result<Self> {
        self.vars
            .insert("messages".to_string(), serde_json::to_value(messages)?);
        Ok(self)
    }
}
