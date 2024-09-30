// Helpers
use std::sync::Mutex;
use tauri::{self, Config, Emitter, Manager};

// IRC
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{RGBColor, ServerMessage};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::message::PrivmsgMessage;

use crate::config::CHANNEL_NAME;

#[derive(serde::Serialize, Clone, Debug)]
pub struct TwitchMessage {
    username: String,
    message: String,
}

// BOT
#[derive(Debug)]
pub struct Bot {
    pub channel_name: String,
    pub client: Mutex<Client>,
    pub chat_messages: Mutex<Vec<TwitchMessage>>
}
impl Bot {
    pub fn connect_to_twitch(&self, app_handle: tauri::AppHandle) {
        println!("Connecting to Twitch!");
        // default configuration is to join chat as anonymous.
        // let config = ClientConfig::default();

        // TODO: Get a configuration file going after collecting info from the user.
        let login_name: String = dotenv!("BOT_NAME").to_owned();
        let oauth_token: String = dotenv!("BOT_OAUTH").to_owned();

        let config = ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(oauth_token)));

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
                        // TODO: Emit join event for the channel as been joined.
                        let _ = app_handle.emit("channel_join", msg.channel_login);
                    }
                    ServerMessage::Part(msg) => {
                        // TODO: Emit part event for the channel as been left.
                        let _ = app_handle.emit("channel_part", msg.channel_login);
                    }
                    ServerMessage::Generic(_) => (),
                    other => {
                        println!("Other message type: {:?}", other)
                    }
                }
            }
        });

        *self.client.lock().unwrap() = Client::new(client);
        println!("Connected to Twitch!");
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
        Bot {
            channel_name: CHANNEL_NAME.into(),
            client: Mutex::new(Client::default()),
            chat_messages: Mutex::new(Vec::new())
        }
    }
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
