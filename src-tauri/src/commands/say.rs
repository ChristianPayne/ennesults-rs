use crate::bot::Bot;

#[tauri::command]
pub async fn say(message: &str, state: tauri::State<'_, Bot>) -> Result<bool, String> {
    let channel_name = state
        .bot_info
        .lock()
        .expect("Failed to get lock")
        .channel_name
        .clone();

    if channel_name.is_empty() {
        return Err("Channel name not found.".into());
    }

    let client;
    {
        client = state.client.lock().unwrap().get_client();
    }

    let Some(client) = client else {
        return Err("Could not get client.".into());
    };

    let channel_name = state
        .bot_info
        .lock()
        .expect("Failed to get lock for bot info")
        .channel_name
        .clone();
    let say_result = client.say(channel_name, message.to_string()).await;
    match say_result {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
