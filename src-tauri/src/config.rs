use crate::api::ullm::UllmApi;
use crate::prelude::*;
use crate::DB;

use std::sync::Arc;

use serde::Deserialize;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::api::Api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UllmConfig {
    pub url: String,
}

impl Default for UllmConfig {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8081".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubConfig {
    UllmDefault(UllmConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub name: String,
    pub subconfig: SubConfig,
    pub r#default: Option<bool>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            name: "µLLM API Default Config".into(),
            subconfig: SubConfig::UllmDefault(UllmConfig::default()),
            r#default: None,
        }
    }
}

impl ApiConfig {
    pub fn into_api(self) -> Result<Arc<Mutex<dyn Api>>> {
        match self.subconfig {
            SubConfig::UllmDefault(config) => Ok(UllmApi::new(config.url)?),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Config;

impl Config {
    pub async fn get_default_api() -> Result<Arc<Mutex<dyn Api>>> {
        let mut apis: Vec<ApiConfig> = db!().select("api").await?;
        if apis.is_empty() {
            let api_config: Option<ApiConfig> = db!()
                .insert(("api", "default"))
                .content(ApiConfig::default())
                .await?;

            let api_config = api_config.ok_or("Failed to insert default api config")?;

            apis.push(api_config);
        }

        let api_config = if apis.len() == 1 {
            apis.pop().ok_or("Failed to get api config")?
        } else {
            apis.into_iter()
                .find(|api_config| api_config.r#default.unwrap_or(false))
                .ok_or("Failed to get default api config")?
        };

        api_config.into_api()
    }
}
