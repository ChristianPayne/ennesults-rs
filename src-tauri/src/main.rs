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

use crate::bot::Bot;

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
            crate::commands::print_state::print_state,
            crate::commands::status::status,
        ])
        .setup(|app| {
            // MOVED ALL FILE RELATED RESEARCH INTO FILE.RS //

            // Get a resource path for where the files will live.
            // let resource_path = app.path_resolver().app_data_dir().expect("Can't resolve app data dir.");
            // let full_path = format!("{}/test.json", resource_path.to_str().expect("Can't convert to str"));

            // println!("Files for the app will be saved here: {}", &full_path);

            // if Path::new(&full_path).exists() == false {
            //   fs::write(&full_path, "{}").expect("Failed to write file.")
            // }

            // let message: String = fs::read_to_string(&full_path).expect("Failed to read string from file path.");

            // dbg!(message);

            let test_data: bool = false;

            file::create_file(
                app.handle(),
                "test_json_file",
                file::to_json(test_data).expect("Failed to convert to json."),
            )
            .expect("failed to create file");

            // app.handle().path_resolver().app_data_dir()

            // let contents = read_string("$DESKTOP/test/test.json");

            // dbg!(contents);

            println!("App Started!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
