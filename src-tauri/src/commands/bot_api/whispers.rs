use tauri::Manager;

use crate::bot::BotData;
use crate::file::{write_file, WriteFileError};

#[tauri::command]
pub fn get_users_allowed_to_whisper(state: tauri::State<'_, BotData>) -> Result<Vec<String>, String> {
  Ok(state.users_allowed_to_whisper.lock().expect("Failed to get lock for users_allowed_to_whisper.").clone())
}

#[tauri::command]
pub fn save_users_allowed_to_whisper(app_handle: tauri::AppHandle, users_allowed_to_whisper: Vec<String>) -> Result<(), String> {
  let state = app_handle.state::<BotData>();
  *state.users_allowed_to_whisper.lock().expect("Failed to get lock for users_allowed_to_whisper") = users_allowed_to_whisper.clone();
  
  let write_result = write_file::<Vec<String>>(&app_handle, "users_allowed_to_whisper.json", users_allowed_to_whisper);

  if let Some(err) = write_result.err() {
    match err {
      WriteFileError::FailedConvertJSON => return Err("Failed to convert to json.".to_string()),
      WriteFileError::FailedCreateFile => return Err("Failed to create file.".to_string()),
      WriteFileError::FailedWriteFile => return Err("Failed to write contents in file.".to_string()),
    }
  } 

  Ok(())
}