// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

// Tauri APIs
use tauri::api::file::read_string;
use std::fs;


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
      // let resource_path = app.path_resolver().resource_dir();
      let resource_path = app.path_resolver().app_data_dir().expect("Can't resolve resource dir.");
      let full_path = format!("{}/test.json", resource_path.to_str().expect("Can't convert to str"));
    
      dbg!(&full_path);
      
      let message: String = fs::read_to_string(&full_path).unwrap();
      
      dbg!(message);
      



      // app.handle().path_resolver().app_data_dir()

      // let contents = read_string("$DESKTOP/test/test.json");

      // dbg!(contents);

      println!("App Started!");
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


