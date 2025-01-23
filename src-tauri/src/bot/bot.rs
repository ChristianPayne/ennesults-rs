use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{self, Emitter, Manager};
use ts_rs::TS;

use std::ops::{Deref, DerefMut};

use crate::bot::{announcement_thread_loop, insult_thread_loop, AnnouncementThread, InsultThread};

use super::{api::get_settings, handle_incoming_chat, BotData, Client, Settings};
use super::{validate_auth, Authentication, AuthenticationDetails};

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
    pub settings: Mutex<Settings>,
    pub auth: Mutex<Authentication>,
    pub bot_data: BotData,
    pub client: Mutex<Client>,
    pub chat_messages: Mutex<Vec<TwitchMessage>>,
}

impl Bot {
    pub fn new(settings: Settings, bot_data: BotData, auth: Authentication) -> Self {
        Self {
            settings: Mutex::new(settings),
            auth: Mutex::new(auth),
            bot_data,
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new()),
        }
    }

    pub fn get_bot_name(&self) -> Option<String> {
        let authentication = self.auth.lock().expect("Failed to get lock for Auth");

        match authentication.clone() {
            Authentication::Valid { details, .. } => Some(details.login),
            Authentication::Invalid { .. } | Authentication::NotSignedIn => None,
        }
    }

    pub fn get_channel_name(&self) -> String {
        self.settings.lock().unwrap().channel_name.clone()
    }

    pub async fn get_channel_status(&self) -> Option<(bool, bool)> {
        let channel_name = self.get_channel_name();
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

    pub fn get_auth_token(&self) -> Option<String> {
        let auth = self.auth.lock().expect("Failed to get lock for auth");

        match auth.clone() {
            Authentication::Valid { details, .. } => Some(details.access_token),
            Authentication::NotSignedIn | Authentication::Invalid { .. } => None,
        }
    }

    pub fn get_client_id(&self) -> Option<String> {
        let auth = self.auth.lock().expect("Failed to get lock for auth");

        match auth.clone() {
            Authentication::Valid { details, .. } => Some(details.client_id),
            Authentication::NotSignedIn | Authentication::Invalid { .. } => None,
        }
    }
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            settings: Mutex::new(Settings::default()),
            auth: Mutex::new(Authentication::default()),
            bot_data: BotData::default(),
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new()),
        }
    }
}

pub mod api {
    use tauri::{AppHandle, Manager};

    use crate::bot::{Bot, TwitchMessage};

    use super::Authentication;

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
    pub fn get_auth_status(app_handle: AppHandle) -> Result<Authentication, String> {
        let state = app_handle.state::<Bot>();
        let auth_guard = state.auth.lock();

        match auth_guard {
            Err(_) => Err("Failed to get lock for Auth".to_string()),
            Ok(auth) => Ok(auth.clone()),
        }
    }
}
