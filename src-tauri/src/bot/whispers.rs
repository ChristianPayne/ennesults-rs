use tauri::{AppHandle, Emitter, Manager};
use twitch_irc::message::WhisperMessage;

use crate::bot::{say, Bot, BotData, BotInfo};

pub async fn handle_whisper(app_handle: AppHandle, msg: WhisperMessage) {
    let bot = app_handle.state::<Bot>();
    let bot_info = app_handle.state::<BotInfo>();

    println!("{} whispered {}", msg.sender.name, msg.message_text);

    {
        let bot_info = bot
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        if !bot_info.enable_whispers {
            return;
        }
    }

    let user_allowed_to_whisper = {
        let users = bot_info.users_allowed_to_whisper.clone();

        users.contains(&msg.sender.name.to_lowercase())
    };

    if user_allowed_to_whisper {
        let _ = say(bot, msg.message_text.as_str()).await;
        app_handle
            .emit(
                "alert",
                format!("{} sent a message through whisper.", msg.sender.name),
            )
            .unwrap();
    } else {
        app_handle
            .emit(
                "alert",
                format!(
                    "{} tried to whisper but was not on the list.",
                    msg.sender.name
                ),
            )
            .unwrap();
    }
}

pub mod api {
    use tauri::Manager;

    use crate::bot::{Bot, BotData};
    use crate::file::{write_file, WriteFileError};

    #[tauri::command]
    pub fn get_users_allowed_to_whisper(
        state: tauri::State<'_, Bot>,
    ) -> Result<Vec<String>, String> {
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");
        Ok(bot_info.users_allowed_to_whisper.clone())
    }
}
