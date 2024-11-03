// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate dotenv_codegen;

//Tauri
use tauri::Manager;
// Ennesults
mod bot;
mod commands;
mod date;
mod file;

use bot::{Bot, BotData, BotInfo, Comebacks, Insults, Users};
use file::read_json_file;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            crate::bot::say,
            crate::bot::api::connect_to_channel,
            crate::bot::api::leave_channel,
            crate::bot::api::get_channel_status,
            crate::bot::api::print_bot_data,
            crate::bot::api::get_channel_name,
            crate::bot::api::save_bot_info,
            crate::bot::api::get_users_allowed_to_whisper,
            crate::bot::api::save_users_allowed_to_whisper,
            crate::bot::api::get_bot_info,
            crate::bot::api::get_chat_messages,
            crate::bot::api::get_chat_messages_count,
            crate::bot::api::get_users,
            crate::bot::api::get_active_users,
            crate::bot::api::delete_user,
        ])
        .setup(|app| {
            println!("Setting up bot!");
            let bot_info =
                read_json_file::<BotInfo>(app.handle(), "bot_info.json").unwrap_or_default();
            let bot = Bot::new(bot_info.clone());
            app.manage(bot);

            let comebacks =
                read_json_file::<Comebacks>(app.handle(), "comebacks.json").unwrap_or_default();
            let insults =
                read_json_file::<Insults>(app.handle(), "insults.json").unwrap_or_default();
            let users = read_json_file::<Users>(app.handle(), "users.json").unwrap_or_default();
            let users_allowed_to_whisper =
                read_json_file::<Vec<String>>(app.handle(), "users_allowed_to_whisper.json")
                    .unwrap_or_default();

            let bot_data = BotData::new(comebacks, insults, users, users_allowed_to_whisper);
            app.manage(bot_data);

            // Connect the bot to Twitch on startup.
            let state = app.state::<Bot>();
            let connection_result = state.connect_to_twitch(app.handle().clone());

            if let Some(error) = connection_result.err() {
                println!("{}", error)
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
