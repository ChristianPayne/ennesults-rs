use crate::bot::{Bot, TwitchMessage};

#[tauri::command]
pub fn get_chat_messages(state: tauri::State<'_, Bot>) -> Result<Vec<TwitchMessage>, String> {
  Ok(state.chat_messages.lock().expect("Failed to get lock for chat messages.").clone())
}
#[tauri::command]
pub fn get_chat_messages_count(state: tauri::State<'_, Bot>) -> Result<usize, String> {
  Ok(state.chat_messages.lock().expect("Failed to get lock for chat messages.").clone().len())
}