use tauri::Manager;

use crate::bot::{Bot, BotInfo};
use crate::file::{write_file, WriteFileError};
use crate::commands::leave_channel;

#[tauri::command]
pub fn get_channel_name(state: tauri::State<'_, Bot>) -> Result<String, String> {
  Ok(state.bot_info.lock().expect("Failed to get lock for bot info").channel_name.clone())
}

#[tauri::command]
pub fn get_bot_info(state: tauri::State<'_, Bot>) -> BotInfo {
  let bot_info = state.bot_info.lock().expect("Failed to get lock for bot info").clone();
  bot_info
}

#[tauri::command]
pub fn save_bot_info(app_handle: tauri::AppHandle, bot_info: BotInfo) -> Result<BotInfo, String> {
  let state = app_handle.state::<Bot>();
  let mut bot_info = bot_info;
  bot_info.channel_name = bot_info.channel_name.to_lowercase();
  {
    *state.bot_info.lock().expect("Failed to get lock for bot info") = bot_info.clone();
  }

  let _ = leave_channel(app_handle.state::<Bot>());
  let _ = state.connect_to_twitch(app_handle.clone());
  
  let write_result = write_file::<BotInfo>(&app_handle, "bot_info.json", bot_info.clone());

  if let Some(err) = write_result.err() {
    return match err {
      WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
      WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
      WriteFileError::FailedWriteFile => Err("Failed to write contents in file.".to_string()),
    }
  } 

  Ok(bot_info)
}