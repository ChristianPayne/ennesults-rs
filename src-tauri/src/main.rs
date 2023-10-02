// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

// TAURI
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_positioner::{WindowExt, Position};

use serde_json::json;

// Ennesults
mod bot;
mod config;
use bot::{Bot};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn say(name: &str, state: tauri::State<'_, Bot>) -> Result<String, ()> {
    if let Some(client) = bot::get_client(&state) {
        //Do something with the client.
        client.say(config::CHANNEL_NAME.to_owned(), name.to_owned()).await;
    };
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}



#[tauri::command]
fn print_state(state: tauri::State<'_, Bot>) {
    dbg!(&state);
    // if let Some(client) = bot::get_client(&state) {
    //     //Do something with the client.
    // }
}


#[tauri::command]
fn leave_channel(state: tauri::State<'_, Bot>) {
    if let Some(client) = bot::get_client(&state) {
        //Do something with the client.
        client.part(config::CHANNEL_NAME.to_owned());
    }
}

#[tauri::command]
async fn connect_to_channel (state: tauri::State<'_, Bot>) -> Result<(), &str> {
    match bot::get_client(&state) {
        None => {
            // Add the client to the shared state.
            Bot::connect_to_twitch(state);
            Ok(())
        },
        Some(client) => {
            // join a channel 
            match client.join(config::CHANNEL_NAME.to_owned()) {
                Ok(_) => {
                    Ok(())
                },
                Err(_e) => Err("Could not join channel!")
            }
        }
    }
}

// #[tauri::command]
// fn connect(connection: State<DbConnection>) {
//   // initialize the connection, mutating the state with interior mutability
//   *connection.db.lock().unwrap() = Some(Connection {});
// }

#[tokio::main]
async fn main() {
    tauri::Builder::default()
      .manage(Bot::default())
      .plugin(tauri_plugin_store::Builder::default().build())
      .invoke_handler(tauri::generate_handler![
        say,
        connect_to_channel,
        leave_channel,
        print_state
      ])
      .setup(|app| {
        // Window position
        let win = app.get_window("main").unwrap();
        let _ = win.move_window(Position::BottomRight);

        // Create store
        let mut store = StoreBuilder::new(app.handle(), "./store.bin".parse()?).build();
        let _ = store.insert("a".to_string(), json!("b"));
        let _ = store.save();

        println!("App Started!");
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}


