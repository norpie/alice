// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::OnceLock;

use anyhow::Result;
use responses::Response;
use serde::Serialize;
use sockets::UllmAPI;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

use crate::responses::SimpleResult;

mod calls;
mod responses;
mod sockets;
mod chat;

static API_URL: &str = "ws://localhost:8081";

static MANAGER: OnceLock<Mutex<UllmAPI>> = OnceLock::new();
static APP: OnceLock<AppHandle> = OnceLock::new();

#[derive(Debug, Clone, Serialize)]
enum ConnectionStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "reconnected")]
    Reconnected,
}

#[derive(Clone, Serialize)]
struct ConnectionStatusEvent {
    status: ConnectionStatus,
}

#[tauri::command]
async fn complete(snippet: String) -> Result<String, String> {
    let manager = MANAGER
        .get()
        .ok_or_else(|| "Failed to get manager".to_string())?;
    let final_tokens = manager
        .lock()
        .await
        .completion(snippet, |tokens| {
            emit_completion_tokens(CompletionTokens { tokens })
        })
        .await
        .map_err(|e| e.to_string())?;
    Ok(final_tokens)
}

#[tauri::command]
async fn check_connection() -> Result<ConnectionStatus, String> {
    let manager = MANAGER
        .get()
        .ok_or_else(|| "Failed to get manager".to_string())?;
    let status = &manager.lock().await.status;
    Ok(match status {
        sockets::ConnectionStatus::Connected => ConnectionStatus::Connected,
        sockets::ConnectionStatus::Disconnected => ConnectionStatus::Disconnected,
    })
}

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
    Ok(response.result.status.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let manager = UllmAPI::new(API_URL).await;
    MANAGER
        .set(Mutex::new(manager))
        .map_err(|_| anyhow::anyhow!("Failed to set manager"))?;
    tauri::Builder::default()
        .setup(|app| {
            APP.set(app.handle().clone()).unwrap();
            tokio::spawn(autorestart());
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![ping, check_connection, complete])
        .run(tauri::generate_context!())
        .map_err(|e| anyhow::anyhow!("Failed to run tauri: {}", e))?;
    MANAGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Failed to get manager"))?
        .lock()
        .await
        .close()
        .await?;
    Ok(())
}

async fn autorestart() {
    emit_connection_status(ConnectionStatus::Disconnected);
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        let mut manager = MANAGER
            .get()
            .ok_or_else(|| anyhow::anyhow!("Failed to get manager"))
            .unwrap()
            .lock()
            .await;
        match manager.status {
            sockets::ConnectionStatus::Disconnected => {
                let res = manager.connect().await;
                if res.is_ok() {
                    manager.status = sockets::ConnectionStatus::Connected;
                    emit_connection_status(ConnectionStatus::Reconnected);
                }
            }
            sockets::ConnectionStatus::Connected => {
                if !manager.is_alive().await {
                    manager.status = sockets::ConnectionStatus::Disconnected;
                    emit_connection_status(ConnectionStatus::Lost);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct CompletionTokens {
    tokens: String,
}

async fn emit_completion_tokens(tokens: CompletionTokens) {
    println!("Emitting completion tokens: {:?}", tokens);
    let res = APP
        .get()
        .ok_or_else(|| anyhow::anyhow!("Failed to get app"))
        .unwrap()
        .emit("completion-tokens", tokens);
    if let Err(e) = res {
        println!("Failed to emit completion tokens: {}", e);
    }
}

fn emit_connection_status(status: ConnectionStatus) {
    println!("Emitting connection status: {:?}", status);
    let payload = ConnectionStatusEvent { status };
    let res = APP
        .get()
        .ok_or_else(|| anyhow::anyhow!("Failed to get app"))
        .unwrap()
        .emit("connection-status", payload);
    if let Err(e) = res {
        println!("Failed to emit connection status: {}", e);
    }
}
