use tauri::{AppHandle, Emitter, Manager};
use twitch_irc::message::WhisperMessage;

use crate::bot::{say, Bot};

pub async fn handle_whisper(app_handle: AppHandle, msg: WhisperMessage) {
    let bot = app_handle.state::<Bot>();

    println!("{} whispered {}", msg.sender.name, msg.message_text);

    let users_allowed_to_whisper = {
        let settings = bot
            .settings
            .lock()
            .expect("Failed to get lock for settings.");

        if !settings.enable_whispers {
            return;
        }

        settings.users_allowed_to_whisper.clone()
    };

    let sender_allowed_to_whisper =
        users_allowed_to_whisper.contains(&msg.sender.name.to_lowercase());

    if sender_allowed_to_whisper {
        let _ = say(app_handle.clone(), msg.message_text.as_str()).await;
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
    use crate::bot::Bot;

    #[tauri::command]
    pub fn get_users_allowed_to_whisper(
        state: tauri::State<'_, Bot>,
    ) -> Result<Vec<String>, String> {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings.");
        Ok(settings.users_allowed_to_whisper.clone())
    }
}
