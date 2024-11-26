use tauri::Emitter;

use crate::{models::model::Model, API_MANAGER, APP};

#[tauri::command]
pub async fn connection_status() -> Result<bool, String> {
    match api!().is_alive().await {
        Ok(bool) => Ok(bool),
        Err(e) => Err(format!("Failed to get connection status: {:?}", e)),
    }
}

#[tauri::command]
pub async fn list_models() -> Result<Vec<Model>, String> {
    let result = api!().list().await;
    match result {
        Ok(models) => {
            println!("Models: {:?}", models);
            Ok(models)
        }
        Err(e) => Err(format!("Failed to list models: {:?}", e)),
    }
}

#[tauri::command]
pub async fn load_model(model: Model) -> Result<(), String> {
    fn preload_callback(status: String) -> crate::prelude::Result<()> {
        app!().emit("model_load", status)?;
        Ok(())
    }
    let result = api!().load(&model, Box::new(preload_callback)).await;
    match result {
        Ok(status) => app!()
            .emit("model_load", status)
            .map_err(|e| format!("Failed to emit: {:?}", e)),
        Err(e) => Err(format!("Error command: {}", e)),
    }
}
