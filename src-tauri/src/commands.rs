use tauri::Emitter;

use crate::{models::model::Model, prelude::*, API_MANAGER, APP};

#[tauri::command]
pub async fn connection_status() -> bool {
    let result = api!().is_alive().await;
    match result {
        Ok(bool) => {
            println!("Connection: {}", bool);
            bool
        }
        Err(_) => {
            eprintln!("Connection check failed");
            false
        }
    }
}

#[tauri::command]
pub async fn list_models() -> Vec<Model> {
    let result = api!().list().await;
    match result {
        Ok(models) => {
            println!("Models: {:?}", models);
            models
        }
        Err(e) => {
            eprintln!("Failed to list models: {:?}", e);
            vec![]
        }
    }
}

#[tauri::command]
pub async fn load_model(model: Model) {
    fn preload_callback(status: String) -> Result<()> {
        app!().emit("model_load", status)?;
        Ok(())
    }
    let result = api!().load(&model, Box::new(preload_callback)).await;
    let status = match result {
        Ok(status) => status,
        Err(e) => {
            eprintln!("Error command: {}", e);
            "error".into()
        }
    };
    // status or "error"
    app!().emit("model_load", status).unwrap();
}
