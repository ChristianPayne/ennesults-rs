use crate::bot::Bot;

#[tauri::command]
pub fn leave_channel(state: tauri::State<'_, Bot>) -> Result<String, String> {
    let channel_name = state
        .bot_info
        .lock()
        .expect("Failed to get lock")
        .channel_name
        .clone();
    let client = state.client.lock().unwrap();
    match &client.0 {
        Some(client) => {
            client.part(channel_name.clone());
            Ok(channel_name)
        }
        None => Err(format!(
            "Failed to leave {}. No client configured.",
            channel_name
        )),
    }
}
