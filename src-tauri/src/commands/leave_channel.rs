use crate::bot;
use crate::bot::Bot;
use crate::config;

#[tauri::command]
fn leave_channel(state: tauri::State<'_, Bot>) {
    if let Some(client) = bot::get_client(&state) {
        //Do something with the client.
        client.part(config::CHANNEL_NAME.to_owned());
        println!("Left channel!");
    }
}