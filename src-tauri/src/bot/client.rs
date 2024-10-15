use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use crate::{
    bot::{process_user_state, Bot, BotData, SerializeRBGColor, TwitchMessage},
    commands::say,
};

use super::handle_whisper;

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
            ServerMessage::Whisper(msg) => handle_whisper(app_handle.clone(), msg).await,
            ServerMessage::RoomState(_) => (),
            other => {
                println!("Other message type: {:?}", other)
            }
        }
    }
}
