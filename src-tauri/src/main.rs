// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use anyhow::Result;
use calls::MethodCall;
use responses::Response;
use serde_json::Value;
use sockets::SocketManager;
use tokio::sync::Mutex;
use uuid::Uuid;

mod calls;
mod responses;
mod sockets;

static MANAGER: OnceLock<Mutex<SocketManager>> = OnceLock::new();

#[tauri::command]
async fn ping() -> Result<String, String> {
    let call: MethodCall<()> = calls::MethodCall {
        id: Uuid::new_v4(),
        method: "ping".to_string(),
        params: None,
    };
    let response: Response<Value> = MANAGER
        .get()
        .unwrap()
        .lock()
        .await
        .call(call)
        .await
        .map_err(|e| e.to_string())?;
    println!("{:?}", response);
    Ok(response.result.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = SocketManager::connect().await;
    MANAGER
        .set(Mutex::new(manager))
        .map_err(|_| anyhow::anyhow!("Failed to set manager"))?;
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ping])
        .run(tauri::generate_context!())
        .map_err(|e| anyhow::anyhow!("Failed to run tauri: {}", e))?;
    MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Failed to get manager"))?
        .lock()
        .await
        .close()
        .await;
    Ok(())
}
