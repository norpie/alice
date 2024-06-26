// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use anyhow::Result;
use responses::Response;
use sockets::ULLMAPI;
use tokio::sync::Mutex;

use crate::responses::SimpleResult;

mod calls;
mod responses;
mod sockets;

static API_URL: &str = "ws://localhost:8081";

static MANAGER: OnceLock<Mutex<ULLMAPI>> = OnceLock::new();

#[tauri::command]
async fn ping() -> Result<String, String> {
    let response: Response<SimpleResult> = MANAGER
        .get()
        .unwrap()
        .lock()
        .await
        .ping()
        .await
        .map_err(|e| e.to_string())?;
    println!("{:?}", response);
    Ok(response.result.status.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = ULLMAPI::new(API_URL).await?;
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
