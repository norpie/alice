use crate::API_MANAGER;

#[tauri::command]
pub async fn connection_status() -> Result<bool, String> {
    match api!().is_alive().await {
        Ok(bool) => Ok(bool),
        Err(e) => Err(format!("Failed to get connection status: {:?}", e)),
    }
}
