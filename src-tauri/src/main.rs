//Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, OnceLock};

use anyhow::Result;
use api::ullm::UllmApi;
use chrono::Utc;
use config::Config;
use manager::Manager;
use models::{message::Message, parameters::EngineParameters};
use surrealdb::{
    engine::local::{Db, RocksDb},
    Surreal,
};
use tauri::AppHandle;
use tokio::sync::Mutex;
use wpp::prompting::Prompt;

macro_rules! app {
    // Use macro to get the app
    () => {
        APP.get().unwrap()
    };
}

macro_rules! api_manager {
    // Use macro to get the manager
    () => {
        API_MANAGER.get().unwrap().lock().await
    };
}

macro_rules! api {
    // Use macro to get the app
    () => {
        api_manager!().api.lock().await
    };
}

macro_rules! db {
    // Use macro to get the app
    () => {
        DB.get().unwrap()
    };
}

mod api;
mod commands;
mod config;
mod events;
mod models;
mod prelude;
mod responses;
// mod sockets;
mod manager;
mod wpp;

static DATA_DIR: &str = "alice";

static APP: OnceLock<AppHandle> = OnceLock::new();
static API_MANAGER: OnceLock<Mutex<Manager>> = OnceLock::new();
static DB: OnceLock<Surreal<Db>> = OnceLock::new();

#[rustfmt::skip]
static LLAMA3_PROMPT_TEMPLATE: &str =
r#"{{{sequence_start}}}{{{system}}}{{{sequence_end}}}

{{{system_prompt}}}{{{suffix}}}{{#each messages}}{{{../sequence_start}}}{{{this.author}}}{{{../sequence_end}}}

{{{this.content}}}{{{../suffix}}}{{/each}}{{{sequence_start}}}{{{next_role}}}{{{sequence_end}}}
"#;

#[tokio::main]
async fn main() -> Result<()> {
    let expanded_data_xdg_data = std::env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        let mut path = std::env::var("HOME").unwrap();
        path.push_str("/.local/share");
        path
    });
    let db_path = format!("{}/{}/db", expanded_data_xdg_data, DATA_DIR);
    let db = Surreal::new::<RocksDb>(db_path).await?;
    db.use_ns("alice").await?;
    db.use_db("local").await?;
    DB.set(db).expect("Failed to set db");

    let api = Config::get_default_api().await?;
    API_MANAGER.set(Manager::new(api)?.into()).expect("Failed to set manager");
    api_manager!().start_keep_alive().await?;

    tauri::Builder::default()
        .setup(|app| {
            APP.set(app.handle().clone()).expect("Failed to set app");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // Connection commands
            commands::connection::connection_status,
            // Model commands
            commands::models::list_models,
            commands::models::load_model
        ])
        .run(tauri::generate_context!())
        .map_err(|e| anyhow::anyhow!("Failed to run tauri: {}", e))?;

    let messages = vec![Message {
        timestamp: Utc::now(),
        author: "user".to_string(),
        content: "Where is the Madou tower?".to_string(),
    }];

    let prompt = Prompt::new(LLAMA3_PROMPT_TEMPLATE.to_string())
        .with_messages(messages)?
        // .with_str_var("bos_token", "<|begin_of_text|>")
        .with_str_var("system", "system")
        .with_str_var("system_prompt", "You are an intelligent assistant.")
        .with_str_var("suffix", "<|eot_id|>")
        .with_str_var("sequence_start", "<|start_header_id|>")
        .with_str_var("sequence_end", "<|end_header_id|>")
        .with_str_var("next_role", "assistant")
        .render()?;

    let mut params = EngineParameters::default();
    params.stop_sequences.push("<|end_of_text|>".to_string());

    // let full = manager!()
    //     .api
    //     .complete(
    //         &prompt,
    //         params,
    //         Box::new(|tokens| {
    //             print!("{}", tokens);
    //             io::stdout().flush().unwrap();
    //             Ok(())
    //         }),
    //     )
    //     .await?;

    api!().disconnect().await?;
    Ok(())
}
