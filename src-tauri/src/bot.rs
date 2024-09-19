// Helpers
use std::sync::Mutex;
use tauri::{self, Emitter, Manager};

// IRC
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{RGBColor, ServerMessage};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

#[derive(serde::Serialize, Clone)]
struct TwitchMessage {
    username: String,
    message: String,
}

// BOT
#[derive(Debug)]
pub struct Bot {
    pub client: Mutex<Client>,
}
impl Bot {
    pub fn connect_to_twitch(app: tauri::AppHandle, state: tauri::State<'_, Bot>) {
        println!("Connecting to Twitch!");
        // default configuration is to join chat as anonymous.
        // let config = ClientConfig::default();

        // TODO: Get a configuration file going after collecting info from the user.
        let login_name: String = dotenv!("BOT_NAME").to_owned();
        let oauth_token: String = dotenv!("BOT_OAUTH").to_owned();

        let config =
            ClientConfig::new_simple(StaticLoginCredentials::new(login_name, Some(oauth_token)));

        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let _join_handle = tokio::spawn(async move {
            // TODO: Figure out how to make this an event emission that rust and js can listen to.
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        println!("Received message: {:?}", msg);
                        let twitch_message = TwitchMessage {
                            username: msg.sender.name,
                            message: msg.message_text,
                        };

                        // self.chat_messages.push(twitch_message.clone());
                        app.emit("message", twitch_message).unwrap();
                    }
                    ServerMessage::Pong(_) => {
                        // println!("Pong received...")
                        ()
                    }
                    ServerMessage::Join(msg) => {
                        // TODO: Emit join event for the channel as been joined.
                        let _ = app.emit("channel_join", msg.channel_login);
                    }
                    ServerMessage::Part(_) => {
                        // TODO: Emit part event for the channel as been left.
                    }
                    ServerMessage::Generic(_) => (),
                    other => {
                        println!("Other message type: {:?}", other)
                    }
                }
            }
        });

        *state.client.lock().unwrap() = Client::new(client);
        println!("Connected to Twitch!");
    }

    // pub fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    //     dbg!(&message, "rs2js");
    //     manager.emit_all("rs2js", message).unwrap();
    // }
}
impl Default for Bot {
    fn default() -> Self {
        Bot {
            client: Mutex::new(Client(None)),
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
pub fn get_client(
    state: &tauri::State<'_, Bot>,
) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
    let mutex_result = &state.client.lock();
    match mutex_result {
        Ok(guard) => guard.0.clone(),
        Err(_) => {
            println!("Error getting client out of mutex!");
            None
        }
    }
}
