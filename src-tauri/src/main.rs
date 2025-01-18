// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
extern crate dotenv_codegen;
use std::sync::Mutex;

use migrations::run_migrations;
//Tauri
use tauri::{Emitter, Manager};

// Ennesults
mod bot;
mod changelog;
mod commands;
mod date;
mod file;
mod migrations;
mod twitch;
mod updater;

use bot::{Announcements, Authentication, Bot, BotData, BotInfo, Comebacks, Insults, Users};
use file::read_json_file;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_oauth::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            crate::bot::say,
            crate::bot::api::connect_to_twitch,
            crate::bot::api::disconnect_from_twitch,
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
            crate::bot::api::update_insult,
            crate::bot::api::save_insults,
            crate::bot::api::delete_insult,
            crate::bot::api::get_announcements,
            crate::bot::api::update_announcement,
            crate::bot::api::delete_announcement,
            crate::bot::api::save_announcements,
            crate::bot::api::open_auth_window,
            crate::bot::api::decode_auth_redirect,
            crate::bot::api::get_auth_status,
            crate::bot::api::sign_out_of_twitch,
            crate::updater::fetch_update,
            crate::updater::install_update,
            crate::changelog::get_changelog
        ])
        .setup(|app| {
            // Manage state for updates.
            app.manage(updater::PendingUpdate(Mutex::new(None)));

            // Run any migrations on the data files before loading the files into the bot.
            println!("🤖 Checking for migrations...");
            run_migrations(app.handle().clone());
            println!("✅ Migrations complete!");

            println!("🤖 Setting up bot...");
            let bot_info =
                read_json_file::<BotInfo>(app.handle(), "bot_info.json").unwrap_or_default();
            let auth =
                read_json_file::<Authentication>(app.handle(), "auth.json").unwrap_or_default();
            let comebacks =
                read_json_file::<Comebacks>(app.handle(), "comebacks.json").unwrap_or_default();
            let insults =
                read_json_file::<Insults>(app.handle(), "insults.json").unwrap_or_default();
            let users = read_json_file::<Users>(app.handle(), "users.json").unwrap_or_default();
            let announcements = read_json_file::<Announcements>(app.handle(), "announcements.json")
                .unwrap_or_default();

            let bot_data = BotData::new(comebacks, insults, users, announcements);
            let bot = Bot::new(bot_info, bot_data, auth);
            app.manage(bot);

            println!("✅ Setup complete!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
