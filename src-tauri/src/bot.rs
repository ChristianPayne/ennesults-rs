pub mod announcements;
pub mod auth;
pub mod bot_data;
pub mod client;
pub mod comebacks;
pub mod corrections;
pub mod insults;
pub mod message_thread;
pub mod settings;
pub mod users;
pub mod whispers;

use crate::bot::{
    auth::Authentication, bot_data::BotData, client::Client, comebacks::Comebacks,
    settings::Settings,
};
use crate::commands::UserLevel;
use std::sync::Mutex;
use ts_rs::TS;

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct TwitchMessage {
    pub message_id: String,
    pub username: String,
    pub message: String,
    pub color: Option<SerializeRBGColor>,
    pub user_level: UserLevel,
    pub timestamp: i64,
}

#[derive(serde::Serialize, Clone, Debug, TS)]
pub struct SerializeRBGColor(pub u8, pub u8, pub u8);

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
#[allow(dead_code)]
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

    pub fn get_bot_name(&self) -> String {
        let authentication = self.auth.lock().expect("Failed to get lock for Auth");

        match authentication.clone() {
            Authentication::Valid { details, .. } => details.login,
            Authentication::Invalid { .. } | Authentication::NotSignedIn => "Ennesults".to_string(),
        }
    }

    pub fn get_channel_name(&self) -> String {
        self.settings.lock().unwrap().channel_name.clone()
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
    pub use super::announcements::api::*;
    pub use super::auth::api::*;
    pub use super::client::api::*;
    pub use super::comebacks::api::*;
    pub use super::insults::api::*;
    pub use super::settings::api::*;
    pub use super::users::api::*;
    pub use super::whispers::api::*;

    use super::Authentication;
    use crate::bot::{Bot, TwitchMessage};
    use tauri::{AppHandle, Manager};

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
