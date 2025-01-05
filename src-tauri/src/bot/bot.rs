use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{self, Emitter, Manager};
use ts_rs::TS;

use std::ops::{Deref, DerefMut};

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
    pub bot_info: Mutex<BotInfo>,
    pub auth: Mutex<AuthValidation>,
    pub bot_data: BotData,
    pub client: Mutex<Client>,
    pub chat_messages: Mutex<Vec<TwitchMessage>>,
}

impl Bot {
    pub fn new(bot_info: BotInfo, bot_data: BotData, auth: AuthValidation) -> Self {
        Self {
            bot_info: Mutex::new(bot_info),
            auth: Mutex::new(auth),
            bot_data,
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new()),
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
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Mutex::new(BotInfo::default()),
            auth: Mutex::new(AuthValidation::default()),
            bot_data: BotData::default(),
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new()),
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
        Self::NotSignedIn
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
