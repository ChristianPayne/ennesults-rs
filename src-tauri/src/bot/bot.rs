use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{self, Emitter, Manager};
use ts_rs::TS;

use std::ops::{Deref, DerefMut};

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::bot::{announcement_thread_loop, insult_thread_loop, AnnouncementThread, InsultThread};

use super::{api::get_bot_info, handle_incoming_chat, BotData, BotInfo, Client};

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct TwitchMessage {
    pub message_id: String,
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
    pub bot_data: BotData,
    pub client: Arc<Mutex<Client>>,
    pub chat_messages: Arc<Mutex<Vec<TwitchMessage>>>,
}

impl Bot {
    pub fn new(bot_info: BotInfo, bot_data: BotData) -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(bot_info)),
            bot_data,
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub async fn get_channel_name(&self) -> String {
        self.bot_info.lock().unwrap().channel_name.clone()
    }
    pub async fn get_channel_status(&self) -> Option<(bool, bool)> {
        let channel_name = self.get_channel_name().await;
        let client = self.client.lock().unwrap();
        match client.deref() {
            Client::Disconnected => None,
            Client::Connected {
                client,
                client_join_handle,
                insult_thread,
                announcement_thread,
            } => Some(client.get_channel_status(channel_name).await),
        }
    }

    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), &str> {
        println!("Connecting to Twitch...");
        let _ = app_handle.emit("alert", "Connecting to Twitch");
        // default configuration is to join chat as anonymous.
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        // Handle the disconnecting of existing client connections to Twitch and any threads that are currently running.
        {
            let existing_client = &mut self
                .client
                .lock()
                .expect("Failed to get lock on bot client");

            match existing_client.deref_mut() {
                Client::Disconnected => {}
                Client::Connected {
                    client,
                    client_join_handle,
                    insult_thread,
                    announcement_thread,
                } => {
                    // Shut down the insult thread if it is running.
                    if let InsultThread::Running {
                        handle: insult_handle,
                        sender: insult_sender,
                    } = insult_thread
                    {
                        insult_sender.send(());
                        insult_handle.abort();
                    }

                    // Shut down the announcement thread if it is running.
                    if let AnnouncementThread::Running {
                        handle: announcement_handle,
                        sender: announcement_sender,
                    } = announcement_thread
                    {
                        announcement_sender.send(());
                        announcement_handle.abort();
                    }

                    // Tell the client to leave the twitch channel.
                    client.part(bot_info.channel_name.clone());

                    // Update the state to reflect the client being disconnected.
                    **existing_client = Client::Disconnected;
                }
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

        let (incoming_messages, twitch_client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // First thing we should do is start consuming incoming messages, otherwise they will back up.
        let twitch_client_thread_handle =
            tokio::spawn(handle_incoming_chat(app_handle.clone(), incoming_messages));

        let insult_thread = InsultThread::new(app_handle.clone(), bot_info.enable_insults);

        let announcement_thread =
            AnnouncementThread::new(app_handle.clone(), bot_info.enable_announcements);

        *self
            .client
            .lock()
            .expect("Failed to get lock for client when connecting to twitch.") = Client::new(
            twitch_client,
            twitch_client_thread_handle,
            insult_thread,
            announcement_thread,
        );

        Ok(())
    }
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(BotInfo::default())),
            bot_data: BotData::default(),
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

pub mod api {
    use crate::bot::{Bot, TwitchMessage};

    #[tauri::command]
    pub fn get_chat_messages(state: tauri::State<'_, Bot>) -> Result<Vec<TwitchMessage>, String> {
        Ok(state
            .chat_messages
            .lock()
            .expect("Failed to get lock for chat messages.")
            .clone())
    }
    #[tauri::command]
    pub fn get_chat_messages_count(state: tauri::State<'_, Bot>) -> Result<usize, String> {
        Ok(state
            .chat_messages
            .lock()
            .expect("Failed to get lock for chat messages.")
            .clone()
            .len())
    }
}
