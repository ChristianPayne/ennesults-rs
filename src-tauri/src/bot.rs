// Helpers
use std::sync::Mutex;
use tauri::{self, Emitter, Manager};

// IRC
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::commands::bot_api::get_bot_info;

#[derive(serde::Serialize, Clone, Debug)]
pub struct TwitchMessage {
    username: String,
    message: String,
}

// BOT
#[derive(Debug)]
pub struct Bot {
    pub bot_info: Mutex<BotInfo>,
    pub client: Mutex<Client>,
    pub chat_messages: Mutex<Vec<TwitchMessage>>
}

impl Bot {
    pub fn new(bot_info: BotInfo) -> Self {
        Self {
            bot_info: Mutex::new(bot_info),
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new())
        }
    }
    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), &str> {
        println!("Connecting to Twitch!");
        let _ = app_handle.emit("alert", "Connecting to Twitch");
        // default configuration is to join chat as anonymous.

        let bot_info = get_bot_info(app_handle.state::<Bot>());

        let config = if bot_info.bot_name == "" || bot_info.oauth_token == "" {
            ClientConfig::default()
        } else {
            ClientConfig::new_simple(StaticLoginCredentials::new(bot_info.bot_name, Some(bot_info.oauth_token)))
        };


        let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let _join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        println!("Received message: {:?}", msg);
                        
                        let twitch_message = TwitchMessage {
                            username: msg.sender.name,
                            message: msg.message_text,
                        };

                        // TODO: Need a lifetime here to be able to hold onto messages.
                        // self.chat_messages.lock().expect("Failed to get lock for chat messages.").push(twitch_message.clone());

                        app_handle.emit("message", twitch_message).unwrap();
                    }
                    ServerMessage::Pong(_) => {
                        // println!("Pong received...")
                        ()
                    }
                    ServerMessage::Join(msg) => {
                        let _ = app_handle.emit("channel_join", msg.channel_login);
                    }
                    ServerMessage::Part(msg) => {
                        // TODO: Emit part event for the channel as been left.
                        let _ = app_handle.emit("channel_part", msg.channel_login);
                    }
                    ServerMessage::Generic(_) => (),
                    ServerMessage::Notice(notice) => {
                        let _ = app_handle.emit("error", notice.message_text);
                        break;
                    },
                    other => {
                        println!("Other message type: {:?}", other)
                    }
                }
            }
        });

        *self.client.lock().unwrap() = Client::new(client);
        Ok(())
    }

    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        let mutex_result = &self.client.lock();
        match mutex_result {
            Ok(guard) => guard.0.clone(),
            Err(_) => {
                println!("Error getting client out of mutex!");
                None
            }
        }
    }

    // pub fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    //     dbg!(&message, "rs2js");
    //     manager.emit_all("rs2js", message).unwrap();
    // }
}
impl Default for Bot {
    fn default() -> Self {
        Self {
            bot_info: Mutex::new(BotInfo::default()),
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new())
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(default = "Default::default")]
pub struct BotInfo {
    pub channel_name: String,
    pub bot_name: String,
    pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub last_comeback_id: u16,
    pub last_insult_id: u16,
    pub last_user_id: u16,
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
            last_user_id: 0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BotData {
    pub comebacks: Mutex<Comebacks>,
    pub insults: Mutex<Insults>,
    pub users: Mutex<Users>,
}

impl BotData {
    pub fn new(comebacks: Comebacks, insults: Insults, users: Users) -> Self {
        Self {
            comebacks: Mutex::new(comebacks),
            insults: Mutex::new(insults),
            users: Mutex::new(users),
        }
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            comebacks: Mutex::new(Comebacks::default()),
            insults: Mutex::new(Insults::default()),
            users: Mutex::new(Users::default()),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(default = "Default::default")]
pub struct Comebacks(pub Vec<Comeback>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Comeback {
    pub id: u16,
    pub value: String
}

impl Default for Comebacks {
    fn default() -> Self {
        Self (Vec::new())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(default = "Default::default")]
pub struct Insults(Vec<Insult>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Insult {
    id: u16,
    value: String
}

impl Default for Insults {
    fn default() -> Self {
        Self (Vec::new())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(default = "Default::default")]
pub struct Users(Vec<User>);

impl Default for Users {
    fn default() -> Self {
        Self (Vec::new())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct User {
    pub id: u16, // Max 65535 users
    pub username: String,
    pub consented: bool,
}

// CLIENT
#[derive(Debug)]
pub struct Client(pub Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>);
impl Client {
    pub fn new(client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>) -> Self {
        Client(Some(client))
    }
}
impl Default for Client {
    fn default() -> Self {
        Client(None)
    }
}
