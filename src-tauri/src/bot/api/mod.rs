mod chat_messages;
mod comebacks;
mod connect_to_channel;
mod get_channel_status;
mod leave_channel;
mod print_bot_data;
mod whispers;

pub use chat_messages::*;
pub use comebacks::*;
pub use connect_to_channel::*;
pub use get_channel_status::*;
pub use leave_channel::*;
pub use print_bot_data::*;
pub use whispers::*;

use tauri::{Emitter, Manager};

use crate::bot::{Bot, BotInfo};
use crate::file::{write_file, WriteFileError};

#[tauri::command]
pub fn get_channel_name(state: tauri::State<'_, Bot>) -> Result<String, String> {
    Ok(state
        .bot_info
        .lock()
        .expect("Failed to get lock for bot info")
        .channel_name
        .clone())
}

#[tauri::command]
pub fn get_bot_info(state: tauri::State<'_, Bot>) -> BotInfo {
    let bot_info = state
        .bot_info
        .lock()
        .expect("Failed to get lock for bot info")
        .clone();
    bot_info
}

#[tauri::command]
pub async fn save_bot_info(
    app_handle: tauri::AppHandle,
    bot_info: BotInfo,
) -> Result<BotInfo, String> {
    let state = app_handle.state::<Bot>();
    let mut bot_info = bot_info;
    bot_info.channel_name = bot_info.channel_name.to_lowercase();
    {
        *state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info") = bot_info.clone();
    }

    let _ = state.connect_to_twitch(app_handle.clone());
    if bot_info.auto_connect_on_startup {
        let _ = connect_to_channel(app_handle.state::<Bot>()).await;
    }

    let write_result = write_file::<BotInfo>(&app_handle, "bot_info.json", bot_info.clone());

    if let Some(err) = write_result.err() {
        return match err {
            WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
            WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
            WriteFileError::FailedWriteFile => Err("Failed to write contents in file.".to_string()),
        };
    }

    let _ = app_handle.emit("bot_info_save", bot_info.clone());

    Ok(bot_info)
}
