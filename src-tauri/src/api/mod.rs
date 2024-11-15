use async_trait::async_trait;

use crate::{
    models::{model::Model, parameters::EngineParameters},
    prelude::*,
};

mod abstractions;
pub mod ullm;

#[async_trait]
pub trait Api: Send + Sync {
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;

    async fn keep_alive(&mut self) -> Result<()>;

    async fn load(&mut self, model: &Model) -> Result<()>;
    async fn unload(&mut self) -> Result<()>;

    async fn list(&mut self) -> Result<Vec<Model>>;

    async fn complete(
        &mut self,
        snippet: &str,
        engine_parameters: EngineParameters,
        streaming_callback: Box<dyn Fn(String) -> Result<()> + Send + Sync>,
    ) -> Result<String>;
}
