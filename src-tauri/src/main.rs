// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

// STD APIs
use std::fs;
use std::path::Path;

// Ennesults
pub mod bot;
pub mod commands;
pub mod config;
pub mod file;

use tauri::Manager;

use crate::bot::Bot;
use crate::commands::connect_to_channel::connect_to_channel;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Bot::default())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            crate::commands::say::say,
            crate::commands::connect_to_channel::connect_to_channel,
            crate::commands::leave_channel::leave_channel,
            crate::commands::print_state::print_state,
            crate::commands::status::status,
        ])
        .setup(|app| {
            println!("Setting up bot!");
            // MOVED ALL FILE RELATED RESEARCH INTO FILE.RS //

            // Connect the bot to Twitch on startup.
            let state = app.state::<Bot>();
            state.connect_to_twitch(app.handle().clone());

            // Not in an async block, we can't kick this off here.
            // More knowledge needed.
            // connect_to_channel(state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
