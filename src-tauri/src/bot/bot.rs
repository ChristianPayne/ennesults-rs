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
    pub auth: Mutex<AuthValidation>,
    pub bot_data: BotData,
    pub client: Arc<Mutex<Client>>,
    pub chat_messages: Arc<Mutex<Vec<TwitchMessage>>>,
}

impl Bot {
    pub fn new(bot_info: BotInfo, bot_data: BotData, auth: AuthValidation) -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(bot_info)),
            auth: Mutex::new(auth),
            bot_data,
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_bot_name(&self) -> Option<String> {
        let auth_guard = self.auth.lock().expect("Failed to get lock for Auth");

        match auth_guard.clone() {
            AuthValidation::Valid { login, .. } => Some(login),
            AuthValidation::Invalid { .. } | AuthValidation::NotSignedIn => None,
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

    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), String> {
        println!("Connecting to Twitch...");
        let state = app_handle.state::<Bot>();
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        // Handle the disconnecting of existing client connections to Twitch and any threads that are currently running.
        self.disconnect_from_twitch(app_handle.clone());

        let auth_guard = state.auth.lock().expect("Failed to get Auth");
        let config = match auth_guard.clone() {
            AuthValidation::Valid {
                access_token,
                login,
                expires_in,
            } => ClientConfig::new_simple(StaticLoginCredentials::new(login, Some(access_token))),
            _ => return Err("Failed to authenticate bot. Auth not valid.".to_string()),
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

    pub fn disconnect_from_twitch(&self, app_handle: tauri::AppHandle) {
        let bot_info = get_bot_info(app_handle.state::<Bot>());
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
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(BotInfo::default())),
            auth: Mutex::new(AuthValidation::default()),
            bot_data: BotData::default(),
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum AuthValidation {
    Valid {
        access_token: String,
        login: String,
        expires_in: i64,
    },
    Invalid {
        reason: String,
    },
    NotSignedIn,
}

impl Default for AuthValidation {
    fn default() -> Self {
        Self::Invalid {
            reason: "Testing Auth".to_string(),
        }
    }
}

pub mod api {
    use crate::bot::{Bot, TwitchMessage};

    use super::AuthValidation;

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

    #[tauri::command]
    pub fn get_auth_status(state: tauri::State<Bot>) -> Result<AuthValidation, String> {
        let auth_guard = state.auth.lock();

        match auth_guard {
            Err(_) => Err("Failed to get lock for Auth".to_string()),
            Ok(auth) => Ok(auth.clone()),
        }
    }
}
