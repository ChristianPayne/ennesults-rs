use crate::bot::Bot;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
    // Is this the best way to get the client? Should we just ignore this command if we don't have a client?
    let channel_name = state.bot_info.lock().expect("Failed to get lock").channel_name.clone();
    match state.get_client() {
        Some(client) => {
            let channel_status = client
                .get_channel_status(channel_name)
                .await;
            dbg!(channel_status);
            Ok(channel_status)
        }
        None => Err("Failed to connect to channel.".to_owned()),
    }
}
