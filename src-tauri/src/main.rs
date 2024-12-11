// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
extern crate dotenv_codegen;
use std::sync::Mutex;

//Tauri
use tauri::{Emitter, Manager};

// Ennesults
mod bot;
mod changelog;
mod commands;
mod date;
mod file;
mod updater;

use bot::{Bot, BotData, BotInfo, Comebacks, Insults, Users};
use file::read_json_file;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            crate::bot::say,
            crate::bot::api::connect_to_channel,
            crate::bot::api::leave_channel,
            crate::bot::api::get_channel_status,
            crate::bot::api::get_channel_name,
            crate::bot::api::save_bot_info,
            crate::bot::api::get_users_allowed_to_whisper,
            crate::bot::api::get_bot_info,
            crate::bot::api::get_chat_messages,
            crate::bot::api::get_chat_messages_count,
            crate::bot::api::get_users,
            crate::bot::api::get_active_users,
            crate::bot::api::delete_user,
            crate::bot::api::get_comebacks,
            crate::bot::api::save_comebacks,
            crate::bot::api::get_comebacks_count,
            crate::bot::api::update_comeback,
            crate::bot::api::delete_comeback,
            crate::bot::api::get_insults,
            crate::bot::api::get_insults_count,
            crate::bot::api::update_comeback,
            crate::bot::api::save_insults,
            crate::bot::api::delete_insult,
            crate::updater::fetch_update,
            crate::updater::install_update,
            crate::changelog::get_changelog,
        ])
        .setup(|app| {
            app.manage(updater::PendingUpdate(Mutex::new(None)));

            println!("Setting up bot!");
            let bot_info =
                read_json_file::<BotInfo>(app.handle(), "bot_info.json").unwrap_or_default();

            let comebacks =
                read_json_file::<Comebacks>(app.handle(), "comebacks.json").unwrap_or_default();
            let insults =
                read_json_file::<Insults>(app.handle(), "insults.json").unwrap_or_default();
            let users = read_json_file::<Users>(app.handle(), "users.json").unwrap_or_default();

            let bot_data = BotData::new(comebacks, insults, users);
            let bot = Bot::new(bot_info, bot_data);
            app.manage(bot);

            // Connect the bot to Twitch on startup.
            let state = app.state::<Bot>();
            if let Err(error) = state.connect_to_twitch(app.handle().clone()) {
                println!("{}", error)
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
