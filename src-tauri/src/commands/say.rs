use crate::bot;
use crate::bot::Bot;
use crate::config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn say(message: &str, state: tauri::State<'_, Bot>) -> Result<String, String> {
    match bot::get_client(&state) {
        Some(client) => {
            let _ = client
                .say(config::CHANNEL_NAME.to_string(), message.to_string())
                .await;
            Ok(message.to_string())
        }
        None => Err("Failed to connect to channel.".to_string()),
    }
}
