// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;
// Ennesults
pub mod bot;
pub mod commands;
pub mod config;
pub mod file;

use crate::bot::Bot;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
      .manage(Bot::default())
      .plugin(tauri_plugin_store::Builder::default().build())
      .plugin(tauri_plugin_window_state::Builder::default().build())
      .invoke_handler(tauri::generate_handler![
        crate::commands::say::say,
        crate::commands::connect_to_channel::connect_to_channel,
        crate::commands::print_state::print_state,
        crate::commands::status::status,
      ])
      .setup(|app| {
        // Create store
        // let mut store = StoreBuilder::new(app.handle(), "./store.bin".parse()?).build();
        // let _ = store.insert("a".to_string(), json!("b"));
        // dbg!(store.is_empty());
        // let _ = store.save();

        // Bot::rs2js("test message".to_string(), &app);

        println!("App Started!");
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}


