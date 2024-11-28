use crate::conversation::{Conversation, LeanConversation};

#[tauri::command]
pub async fn new_conversation() -> Result<Conversation, String> {
    Ok(Conversation::new().await?)
}

#[tauri::command]
pub async fn conversations_date_sorted(
    limit: usize,
    offset: usize,
) -> Result<Vec<LeanConversation>, String> {
    Ok(Conversation::date_sorted_lean(limit, offset).await?)
}

#[tauri::command]
pub async fn find_conversation(id: String) -> Result<Conversation, String> {
    Ok(Conversation::find(id).await?)
}

#[tauri::command]
pub async fn set_conversation_name(id: String, name: String) -> Result<Conversation, String> {
    let conv = Conversation::find(id).await?;
    Ok(conv.with_name(name).await?)
}

#[tauri::command]
pub async fn new_message(
    id: String,
    role: String,
    message: String,
) -> Result<Conversation, String> {
    let conv = Conversation::find(id).await?;
    Ok(conv.with_message(role, message).await?)
}

#[tauri::command]
pub async fn delete_message(id: String, index: usize) -> Result<Conversation, String> {
    let conv = Conversation::find(id).await?;
    Ok(conv.without_message(index).await?)
}

#[tauri::command]
pub async fn with_replaced_message(
    id: String,
    index: usize,
    content: String,
) -> Result<Conversation, String> {
    let conv = Conversation::find(id).await?;
    Ok(conv.with_replaced_message(index, content).await?)
}
