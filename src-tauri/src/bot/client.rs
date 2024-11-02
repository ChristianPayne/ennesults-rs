use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use super::{handle_whisper, process_user_state, Bot, BotData, SerializeRBGColor, TwitchMessage};
use crate::commands::{meets_minimum_user_level, parse_for_command, parse_msg_for_user_level};

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

#[tauri::command]
pub async fn say(message: &str, state: tauri::State<'_, Bot>) -> Result<bool, String> {
    let channel_name = state
        .bot_info
        .lock()
        .expect("Failed to get lock")
        .channel_name
        .clone();

    if channel_name.is_empty() {
        return Err("Channel name not found.".into());
    }

    let client;
    {
        client = state.client.lock().unwrap().get_client();
    }

    let Some(client) = client else {
        return Err("Could not get client.".into());
    };

    let channel_name = state
        .bot_info
        .lock()
        .expect("Failed to get lock for bot info")
        .channel_name
        .clone();
    let say_result = client.say(channel_name, message.to_string()).await;
    match say_result {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn handle_incoming_chat(
    app_handle: AppHandle,
    mut incoming_messages: UnboundedReceiver<ServerMessage>,
) {
    while let Some(message) = incoming_messages.recv().await {
        let bot = app_handle.state::<Bot>();
        match message {
            ServerMessage::Privmsg(msg) => {
                {
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
                }

                process_user_state(app_handle.clone(), &msg.sender);

                match parse_for_command(&msg) {
                    Err(_) => (),
                    Ok((command, args)) => {
                        if !meets_minimum_user_level(
                            parse_msg_for_user_level(&msg),
                            command.get_required_user_level(),
                        ) {
                            let _ = say(
                                "You do not have access to that command.",
                                app_handle.state::<Bot>(),
                            )
                            .await;

                            continue;
                        }

                        match command.run(args, &msg, app_handle.clone()) {
                            None => (),
                            Some(reply) => {
                                // say back the reply.
                                let _ = say(reply.as_str(), app_handle.state::<Bot>()).await;
                            }
                        }
                    }
                }
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
