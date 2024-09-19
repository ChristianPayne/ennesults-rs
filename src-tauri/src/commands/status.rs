use crate::bot;
use crate::bot::Bot;
use crate::config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
    // Is this the best way to get the client? Should we just ignore this command if we don't have a client?
    match bot::get_client(&state) {
        Some(client) => {
            let channel_status = client
                .get_channel_status(config::CHANNEL_NAME.to_owned())
                .await;
            dbg!(channel_status);
            Ok(channel_status)
        }
        None => Err("Failed to connect to channel.".to_owned()),
    }
}
