use crate::bot::Bot;

#[tauri::command]
pub fn leave_channel(state: tauri::State<'_, Bot>) {
    let channel_name = state.bot_info.lock().expect("Failed to get lock").channel_name.clone();
    if let Some(client) = state.get_client() {
        //Do something with the client.
        client.part(channel_name);
        println!("Left channel!");
    }
}
