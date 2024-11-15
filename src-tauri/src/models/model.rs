use core::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub engine: Engine,
}
impl Model {
    pub fn new(name: String, engine: Engine) -> Self {
        Self { name, engine }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Engine {
    #[serde(rename = "llama-cpp")]
    LlamaCpp,
    #[serde(rename = "exllamav2")]
    ExllamaV2,
    #[serde(rename = "transformers")]
    Transformers,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Engine::LlamaCpp => write!(f, "LlamaCpp"),
            Engine::ExllamaV2 => write!(f, "ExllamaV2"),
            Engine::Transformers => write!(f, "Transformers"),
        }
    }
}
