use crate::bot::Bot;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn say(message: &str, state: tauri::State<'_, Bot>) -> Result<bool, String> {
    match state.get_client() {
        Some(client) => {
            let channel_name = state.bot_info.lock().expect("Failed to get lock for bot info").channel_name.clone();
            let say_result = client
                .say(channel_name, message.to_string())
                .await;
            match say_result {
                Ok(_) => Ok(true),
                Err(e) => Err(e.to_string())
            }
        }
        None => Err("Failed to connect to channel.".to_string()),
    }
}
