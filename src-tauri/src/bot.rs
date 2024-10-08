// Helpers
use std::sync::{Arc, Mutex};
use serde::ser::SerializeStruct;
use tauri::{self, Emitter, Manager};

use tokio::task::JoinHandle;
// use tokio::sync::Mutex;
// IRC
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{RGBColor, ServerMessage};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::commands::bot_api::get_bot_info;

#[derive(serde::Serialize, Clone, Debug)]
pub struct TwitchMessage {
    username: String,
    message: String,
    color: Option<SerializeRBGColor>
}

#[derive(Clone, Debug)]
pub struct SerializeRBGColor(RGBColor);

impl serde::Serialize for SerializeRBGColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                let mut state = serializer.serialize_struct("RGBColor", 3)?;
                state.serialize_field("r", &self.0.r)?;
                state.serialize_field("g", &self.0.g)?;
                state.serialize_field("b", &self.0.b)?;
                state.end()
    }
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
            Some(client) => Some(client.get_channel_status(channel_name).await)
        }
    }

    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) -> Result<(), &str> {
        println!("Connecting to Twitch!");
        let _ = app_handle.emit("alert", "Connecting to Twitch");
        // default configuration is to join chat as anonymous.

        // let current_client = *self.client.lock().unwrap();

        // if let Some(client) = &self.client.lock().expect("Failed to get lock on bot client").0 {
        //     println!("Dropped client that was already there.");
        //     let _ = drop(*client);
        // }

        let bot_info = get_bot_info(app_handle.state::<Bot>());

        let config = if bot_info.bot_name == "" || bot_info.oauth_token == "" {
            ClientConfig::default()
        } else {
            ClientConfig::new_simple(StaticLoginCredentials::new(bot_info.bot_name, Some(bot_info.oauth_token)))
        };

        let (mut incoming_messages, client) = TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        // println!("Received message: {:?}", msg);
                        
                        let twitch_message = TwitchMessage {
                            username: msg.sender.name,
                            message: msg.message_text,
                            color: match msg.name_color {
                                Some(rgb) => Some(SerializeRBGColor(rgb)),
                                None => None
                            }
                        };

                        // TODO: Need a lifetime here to be able to hold onto messages.
                        // self.chat_messages.lock().expect("Failed to get lock for chat messages.").push(twitch_message.clone());

                        app_handle.emit("message", twitch_message).unwrap();
                    },
                    ServerMessage::GlobalUserState(user_state) => {
                        ()
                    },
                    ServerMessage::Pong(_) => {
                        // println!("Pong received...")
                        ()
                    },
                    ServerMessage::Join(msg) => {
                        let _ = app_handle.emit("channel_join", msg.channel_login);
                    },
                    ServerMessage::Part(msg) => {
                        // TODO: Emit part event for the channel as been left.
                        let _ = app_handle.emit("channel_part", msg.channel_login);
                    },
                    ServerMessage::Generic(_) => (),
                    ServerMessage::Notice(notice) => {
                        let _ = app_handle.emit("error", notice.message_text);
                    },
                    ServerMessage::Whisper(msg) => {
                        // TODO: Figure out how to implement Sync for a Mutex.
                        // let bot_data = app_handle.state::<BotData>();
                        // let users = &bot_data.users_allowed_to_whisper.lock().expect("Failed to get lock for bot data.");

                        // let mut matched_user: Option<User> = None;
                        // users.0.iter().map(|user| {
                        //     if user.id.to_string() == msg.sender.id {
                        //         matched_user = Some(user.clone())
                        //     }
                        // });

                        // match matched_user {
                        //     None => (),
                        //     Some(user) => {
                        //         say(msg.message_text.as_str(), app_handle.state::<Bot>()).await;
                        //         ()
                        //     }
                        // }
                        
                    }
                    other => {
                        println!("Other message type: {:?}", other)
                    }
                }
            }
        });

        *self.client.lock().unwrap() = Client::new(client, join_handle);
        Ok(())
    }

    // pub fn get_client(&self) -> Option<Client> {
    //     // LEFT OFF HERE TRYING TO FIGURE OUT MUTEX GUARDS AND HOW I CAN GET DATA OUT OF THEM.
    //     let mutex_result = &self.client.lock().expect("Failed to get lock on client.");
    //     mutex_result.0
    // }

    // pub fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    //     dbg!(&message, "rs2js");
    //     manager.emit_all("rs2js", message).unwrap();
    // }
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(default = "Default::default")]
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
pub struct Client(pub Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>, Option<JoinHandle<()>>);

impl Client {
    pub fn new(client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>, join_handle: JoinHandle<()>) -> Self {
        Client(Some(client), Some(join_handle))
    }
    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        match &self.0 {
            None => None,
            Some(client) => Some(client.clone())
        }
    }
}
impl Default for Client {
    fn default() -> Self {
        Client(None, None)
    }
}
