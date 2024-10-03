use tauri::Manager;

use crate::bot::BotData;
use crate::bot::BotInfo;
use crate::bot::Bot;

use crate::bot::Comebacks;
use crate::file::{write_file, WriteFileError};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
pub fn save_bot_info(app_handle: tauri::AppHandle, bot_info: BotInfo) -> Result<(), String> {
  let state = app_handle.state::<Bot>();
  let mut bot_info = bot_info;
  bot_info.channel_name = bot_info.channel_name.to_lowercase();
  *state.bot_info.lock().expect("Failed to get lock for bot info") = bot_info.clone();
  let _ = state.connect_to_twitch(app_handle.clone());
  
  let write_result = write_file::<BotInfo>(&app_handle, "bot_info.json", bot_info);

  if let Some(err) = write_result.err() {
    match err {
      WriteFileError::FailedConvertJSON => return Err("Failed to convert to json.".to_string()),
      WriteFileError::FailedCreateFile => return Err("Failed to create file.".to_string()),
      WriteFileError::FailedWriteFile => return Err("Failed to write contents in file.".to_string()),
    }
  } 

  Ok(())
}
#[tauri::command]
pub fn save_comebacks(app_handle: tauri::AppHandle, comebacks: Comebacks) -> Result<(), String> {
  let state = app_handle.state::<BotData>();
  *state.comebacks.lock().expect("Failed to get lock for bot info") = comebacks.clone();
  
  let write_result = write_file::<Comebacks>(&app_handle, "comebacks.json", comebacks);

  if let Some(err) = write_result.err() {
    match err {
      WriteFileError::FailedConvertJSON => return Err("Failed to convert to json.".to_string()),
      WriteFileError::FailedCreateFile => return Err("Failed to create file.".to_string()),
      WriteFileError::FailedWriteFile => return Err("Failed to write contents in file.".to_string()),
    }
  } 

  Ok(())
}