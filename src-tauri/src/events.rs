use crate::prelude::*;

use tauri::Emitter;

use crate::APP;

pub async fn emit_connection_status(is_alive: bool) -> Result<()> {
    Ok(app!().emit("connection_status", is_alive)?)
}
