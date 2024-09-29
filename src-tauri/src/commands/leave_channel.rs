use crate::bot;
use crate::bot::Bot;
use crate::config;

#[tauri::command]
pub fn leave_channel(state: tauri::State<'_, Bot>) {
    if let Some(client) = state.get_client() {
        //Do something with the client.
        client.part(config::CHANNEL_NAME.to_owned());
        println!("Left channel!");
    }
}
