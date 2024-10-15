use tokio::task::JoinHandle;

use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::message::ServerMessage;

use crate::bot::process_user_state;
use crate::{
    bot::{Bot, BotData, BotInfo, SerializeRBGColor, TwitchMessage, User},
    commands::say,
};

// CLIENT
#[derive(Debug, Default)]
pub struct Client(
    pub Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>>,
    pub Option<JoinHandle<()>>,
);

impl Client {
    pub fn new(
        client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        join_handle: JoinHandle<()>,
    ) -> Self {
        Client(Some(client), Some(join_handle))
    }
    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        self.0.clone()
    }
}

pub async fn process_twitch_messages(
    app_handle: AppHandle,
    mut incoming_messages: UnboundedReceiver<ServerMessage>,
) {
    while let Some(message) = incoming_messages.recv().await {
        let bot = app_handle.state::<Bot>();
        let bot_data = app_handle.state::<BotData>();

        match message {
            ServerMessage::Privmsg(msg) => {
                // println!("Received message: {:?}", msg);

                let mut chat_messages = bot
                    .chat_messages
                    .lock()
                    .expect("Failed to get lock for chat_messages on bot state.");

                let twitch_message = TwitchMessage {
                    username: msg.sender.name.clone(),
                    message: msg.message_text.clone(),
                    color: msg
                        .name_color
                        .map(|color| SerializeRBGColor(color.r, color.g, color.b)),
                };

                chat_messages.push(twitch_message.clone());

                app_handle.emit("message", twitch_message).unwrap();

                process_user_state(app_handle.clone(), &msg.sender)
            }
            ServerMessage::GlobalUserState(_) => (),
            ServerMessage::Pong(_) => (),
            ServerMessage::Join(msg) => {
                let _ = app_handle.emit("channel_join", msg.channel_login);
            }
            ServerMessage::Part(_) => (),
            ServerMessage::Generic(_) => (),
            ServerMessage::UserNotice(_) => (),
            ServerMessage::UserState(_) => (),
            ServerMessage::Notice(notice) => {
                let _ = app_handle.emit("error", notice.message_text);
            }
            ServerMessage::Whisper(msg) => {
                println!("{} whispered {}", msg.sender.name, msg.message_text);

                {
                    let bot_info = bot
                        .bot_info
                        .lock()
                        .expect("Failed to get lock for bot info.");

                    if !bot_info.enable_whispers {
                        return;
                    }
                }

                let user_allowed_to_whisper = {
                    let users = bot_data
                        .users_allowed_to_whisper
                        .lock()
                        .expect("Failed to get lock for bot data.");

                    users.contains(&msg.sender.name.to_lowercase())
                };

                if user_allowed_to_whisper {
                    let _ = say(msg.message_text.as_str(), bot).await;
                    app_handle
                        .emit(
                            "alert",
                            format!("{} sent a message through whisper.", msg.sender.name),
                        )
                        .unwrap();
                } else {
                    app_handle
                        .emit(
                            "alert",
                            format!(
                                "{} tried to whisper but was not on the list.",
                                msg.sender.name
                            ),
                        )
                        .unwrap();
                }
            }
            ServerMessage::RoomState(_) => (),
            other => {
                println!("Other message type: {:?}", other)
            }
        }
    }
}
