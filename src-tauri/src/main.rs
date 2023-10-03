// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate dotenv_codegen;

// TAURI
// use tauri::Manager;
use tauri_plugin_store::StoreBuilder;
// use tauri_plugin_positioner::{WindowExt, Position};

use serde_json::json;

// Ennesults
mod bot;
mod config;
use bot::{Bot};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn say(message: &str, state: tauri::State<'_, Bot>) -> Result<String, String> {
    match bot::get_client(&state) {
        Some(client) => {
            let _ = client.say(config::CHANNEL_NAME.to_owned(), message.to_owned()).await;
            Ok(message.clone().to_owned())
        },
        None => Err("Failed to connect to channel.".to_owned())
    }
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
    // Is this the best way to get the client? Should we just ignore this command if we don't have a client?
    match bot::get_client(&state) {
        Some(client) => {
            let channel_status = client.get_channel_status(config::CHANNEL_NAME.to_owned()).await;
            dbg!(channel_status);
            Ok(channel_status)
        },
        None => Err("Failed to connect to channel.".to_owned())
    }
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
        println!("Left channel!");
    }
}


// TODO: As it stands, when the connection is first initiated, there is no client so we don't join the channel, we just connect to twitch. Then we need to click it again to join.
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
                    println!("Connected to channel!");
                    Ok(())
                },
                Err(_e) => Err("Could not join channel!")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
      .manage(Bot::default())
      .plugin(tauri_plugin_store::Builder::default().build())
      .plugin(tauri_plugin_window_state::Builder::default().build())
      .invoke_handler(tauri::generate_handler![
        say,
        connect_to_channel,
        leave_channel,
        print_state,
        status
      ])
      .setup(|app| {
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


