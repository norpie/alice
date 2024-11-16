use crate::API_MANAGER;

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
