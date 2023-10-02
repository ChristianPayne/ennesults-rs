// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

// TAURI
use tauri::Manager;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_positioner::{WindowExt, Position};

use tokio::task::JoinHandle;
use twitch_irc::message::ServerMessage;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};

use serde_json::json;

// Ennesults
mod bot;
use bot::{Bot};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



#[tauri::command]
fn print_state(state: tauri::State<'_, Bot>) {
    dbg!(&state);
    if let Some(client) = bot::get_client(&state) {
        //Do something with the client.
    }
}

#[tauri::command]
async fn connect_to_channel (app_handle: tauri::AppHandle, state: tauri::State<'_, Bot>) -> Result<(), ()> {
    // dbg!(app_handle.path_resolver().app_data_dir());
    // let client = *state;//.client.lock().unwrap();
    println!("Connection connecting!");
    // default configuration is to join chat as anonymous.
    let config = ClientConfig::default();
    let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => println!("Received message: {:?}", msg.message_text),
                _ => ()
            }
        }
    });

    // join a channel 
    let _ = client.join("ennegineer".to_owned());

    {
        *state.client.lock().unwrap() = bot::Client::new(client);
    }

    // *state.client = client;

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
    Ok(())
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
        greet,
        connect_to_channel,
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


