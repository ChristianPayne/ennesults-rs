use crate::bot;
use crate::bot::Bot;
use crate::config;

#[tauri::command]
pub fn print_state(state: tauri::State<'_, Bot>) {
    dbg!(&state);
    // if let Some(client) = bot::get_client(&state) {
    //     //Do something with the client.
    // }
}