use std::sync::{Arc, Mutex};
use tauri::{self, Emitter, Manager};
use ts_rs::TS;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use super::api::get_bot_info;
use super::{handle_incoming_chat, BotInfo, Client};

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct TwitchMessage {
    pub username: String,
    pub message: String,
    pub color: Option<SerializeRBGColor>,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
pub struct SerializeRBGColor(pub u8, pub u8, pub u8);

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum Alert {
    /// System messages that are ephemeral
    System,
    /// Normal messages given to the streamer.
    Info,
    /// Warnings that something happened.
    Warn,
    /// Recoverable errors that happened in the app.
    Error,
}

// BOT
#[derive(Debug)]
pub struct Bot {
    pub bot_info: Arc<Mutex<BotInfo>>,
    pub client: Arc<Mutex<Client>>,
    pub chat_messages: Arc<Mutex<Vec<TwitchMessage>>>,
}

impl Bot {
    pub fn new(bot_info: BotInfo) -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(bot_info)),
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub async fn get_channel_name(&self) -> String {
        self.bot_info.lock().unwrap().channel_name.clone()
    }
    pub async fn get_channel_status(&self) -> Option<(bool, bool)> {
        let client = self.client.lock().unwrap();
        let channel_name = self.get_channel_name().await;
        match &client.0 {
            None => None,
            Some(client) => Some(client.get_channel_status(channel_name).await),
        }
    }

    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), &str> {
        println!("Connecting to Twitch!");
        let _ = app_handle.emit("alert", "Connecting to Twitch");
        // default configuration is to join chat as anonymous.
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        {
            let existing_client = &mut self
                .client
                .lock()
                .expect("Failed to get lock on bot client");

            if existing_client.0.is_some() {
                println!("Dropped client that was already there.");
                existing_client
                    .0
                    .take()
                    .expect("Failed to take existing client")
                    .part(bot_info.channel_name.clone());

                let existing_handle = existing_client
                    .1
                    .take()
                    .expect("Failed to take existing client handle.");

                existing_handle.abort();
            }
        }

        let config = if bot_info.bot_name.is_empty() || bot_info.oauth_token.is_empty() {
            ClientConfig::default()
        } else {
            ClientConfig::new_simple(StaticLoginCredentials::new(
                bot_info.bot_name,
                Some(bot_info.oauth_token),
            ))
        };

        let (incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(handle_incoming_chat(app_handle.clone(), incoming_messages));

        *self.client.lock().unwrap() = Client::new(client, join_handle);

        Ok(())
    }
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(BotInfo::default())),
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
