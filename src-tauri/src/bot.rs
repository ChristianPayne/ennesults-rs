use std::sync::{Arc, Mutex};
use serde::ser::SerializeStruct;
use tauri::{self, Emitter, Manager};
use ts_rs::TS;

use tokio::task::JoinHandle;
// use tokio::sync::Mutex;
// IRC
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{RGBColor, ServerMessage};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::commands::get_bot_info;
use crate::twitch_worker::process_twitch_messages;

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to="../../src/lib/types.ts")]
pub struct TwitchMessage {
    pub username: String,
    pub message: String,
    pub color: Option<SerializeRBGColor>
}

#[derive(serde::Serialize, Clone, Debug, TS)]
pub struct SerializeRBGColor(pub u8,pub u8,pub u8);
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
            Some(client) => Some(client.get_channel_status(channel_name).await)
        }
    }

    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), &str> {
        println!("Connecting to Twitch!");
        let _ = app_handle.emit("alert", "Connecting to Twitch");
        // default configuration is to join chat as anonymous.
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        {
            let existing_client = &mut self.client.lock().expect("Failed to get lock on bot client").0;

            if existing_client.is_some() {
                println!("Dropped client that was already there.");
                existing_client.take().expect("Failed to take the client").part(bot_info.channel_name.clone());
            }
        }

        let config = if bot_info.bot_name.is_empty() || bot_info.oauth_token.is_empty() {
            ClientConfig::default()
        } else {
            ClientConfig::new_simple(StaticLoginCredentials::new(bot_info.bot_name, Some(bot_info.oauth_token)))
        };

        let (incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(process_twitch_messages(app_handle.clone(), incoming_messages));

        *self.client.lock().unwrap() = Client::new(client, join_handle);
        Ok(())
    }
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Arc::new(Mutex::new(BotInfo::default())),
            client: Arc::new(Mutex::new(Client::default())),
            chat_messages: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[serde(default = "Default::default")]
#[ts(export, export_to="../../src/lib/types.ts")]
pub struct BotInfo {
    pub channel_name: String,
    pub bot_name: String,
    pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub last_comeback_id: u16,
    pub last_insult_id: u16,
}

impl Default for BotInfo {
    fn default() -> Self {
        Self {
            channel_name: "".into(),
            bot_name: "".into(),
            oauth_token: "".into(),
            auto_connect_on_startup: false,
            last_comeback_id: 0,
            last_insult_id: 0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BotData {
    pub comebacks: Mutex<Comebacks>,
    pub insults: Mutex<Insults>,
    pub users: Mutex<Users>,
    pub users_allowed_to_whisper: Mutex<Users>
}

impl BotData {
    pub fn new(comebacks: Comebacks, insults: Insults, users: Users, users_allowed_to_whisper: Users) -> Self {
        Self {
            comebacks: Mutex::new(comebacks),
            insults: Mutex::new(insults),
            users: Mutex::new(users),
            users_allowed_to_whisper: Mutex::new(users_allowed_to_whisper)
        }
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            comebacks: Mutex::new(Comebacks::default()),
            insults: Mutex::new(Insults::default()),
            users: Mutex::new(Users::default()),
            users_allowed_to_whisper: Mutex::new(Users::default())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Comebacks(pub Vec<Comeback>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Comeback {
    pub id: u16,
    pub value: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Insults(Vec<Insult>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Insult {
    id: u16,
    value: String
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Users(pub Vec<User>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub consented: bool,
}

// CLIENT
#[derive(Debug, Default)]
pub struct Client(pub Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>, Option<JoinHandle<()>>);

impl Client {
    pub fn new(client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>, join_handle: JoinHandle<()>) -> Self {
        Client(Some(client), Some(join_handle))
    }
    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        self.0.clone()
    }
}