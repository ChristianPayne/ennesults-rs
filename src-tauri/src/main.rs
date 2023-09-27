// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::Mutex;

use tauri::Manager;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use tauri_plugin_positioner::{WindowExt, Position};

use serde::ser::{Serialize, Serializer};


pub struct BotState(pub Mutex<Bot>);

impl Default for BotState {
    fn default() -> Self { 
        BotState (
            Mutex::new(Bot::default())
        )
    }
}
pub struct Bot {
    client: Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>
}

impl Default for Bot {
    fn default() -> Self { 
        Bot {
            client: None
        }
    }
}


impl Bot {
    pub async fn initialize_bot (mut self) {
        println!("Bot initializing!");
        // default configuration is to join chat as anonymous.
        let config = ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    
        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                println!("Received message: {:?}", message);
            }
        });
    
        // join a channel
        // This function only returns an error if the passed channel login name is malformed,
        // so in this simple case where the channel name is hardcoded we can ignore the potential
        // error with `unwrap`.
        self.client = match client.join("ennegineer".to_owned()) {
            Ok(()) => Some(client),
            _ => None
        };
    
        // keep the tokio executor alive.
        // If you return instead of waiting the background task will exit.
        join_handle.await.unwrap();
    }
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn connect_to_channel (state: tauri::State<BotState>) {
    // let state_guard = state.0.lock().unwrap();
    // state_guard.initialize_bot();
    // tokio::spawn(async move {
    // });
}

#[tokio::main]
async fn main() {
    println!("App Started!");
    tauri::Builder::default()
      .manage(BotState(Default::default()))
      .invoke_handler(tauri::generate_handler![
        greet,
        connect_to_channel
      ])
      .setup(|app| {
        let win = app.get_window("main").unwrap();
        let _ = win.move_window(Position::BottomRight);
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}


