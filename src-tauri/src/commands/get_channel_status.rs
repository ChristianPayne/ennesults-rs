use crate::bot::Bot;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn get_channel_status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
    let channel_name = state.bot_info.lock().expect("Failed to get lock").channel_name.clone();

    if channel_name.is_empty() {
        return Err("Channel name not found.".into())
    }

    let client;
    {
        client = state.client.lock().unwrap().get_client();
    }

    let Some(client) = client else {
        return Err("Could not get client.".into())
    };

    let channel_status = client.get_channel_status(channel_name).await;
    dbg!(channel_status);
    Ok(channel_status)
}
