use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use super::{
    handle_whisper, process_comebacks, process_corrections, process_user_state, Bot, BotData,
    SerializeRBGColor, TwitchMessage,
};
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
pub async fn say(state: tauri::State<'_, Bot>, message: &str) -> Result<(), String> {
    let channel_name = {
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        if bot_info.channel_name.is_empty() {
            return Err("Channel name not found.".into());
        }
        bot_info.channel_name.clone()
    };

    let Some(client) = ({
        state
            .client
            .lock()
            .expect("Failed to get client lock.")
            .get_client()
    }) else {
        return Err("Could not get client.".into());
    };

    if let Err(e) = client.say(channel_name, message.to_string()).await {
        return Err(e.to_string());
    }

    Ok(())
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

                let comeback_said = process_comebacks(app_handle.clone(), &msg).await;

                if !comeback_said {
                    process_corrections(app_handle.clone(), &msg).await;
                }

                if let Ok((command, args)) = parse_for_command(&msg) {
                    if meets_minimum_user_level(
                        parse_msg_for_user_level(&msg),
                        command.get_required_user_level(),
                    ) {
                        if let Some(reply) = command.run(args, &msg, app_handle.clone()) {
                            // say back the reply.
                            let _ = say(app_handle.state::<Bot>(), reply.as_str()).await;
                        }
                    } else {
                        let _ = say(
                            app_handle.state::<Bot>(),
                            "You do not have access to that command.",
                        )
                        .await;
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

pub mod api {
    use crate::bot::Bot;
    use tauri::{AppHandle, Emitter};

    #[tauri::command]
    pub async fn connect_to_channel(state: tauri::State<'_, Bot>) -> Result<String, String> {
        let channel_name = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info")
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

        let channel_status = client.get_channel_status(channel_name.clone()).await;

        match channel_status {
            (true, false) => Err("Already joining a channel.".into()),
            (true, true) => Err("Already connected to a channel.".into()),
            _ => {
                // join a channel
                match client.join(channel_name.clone()) {
                    Ok(x) => {
                        println!("Connected to channel! {:?}", x);
                        Ok(channel_name.clone())
                    }
                    Err(e) => Err(format!("Could not join channel! {}", e)),
                }
            }
        }
    }

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    #[tauri::command]
    pub async fn get_channel_status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
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

        let channel_status = client.get_channel_status(channel_name).await;
        Ok(channel_status)
    }

    #[tauri::command]
    pub fn leave_channel(
        app_handle: AppHandle,
        state: tauri::State<'_, Bot>,
    ) -> Result<String, String> {
        let channel_name = state
            .bot_info
            .lock()
            .expect("Failed to get lock")
            .channel_name
            .clone();
        let client = state.client.lock().unwrap();
        match &client.0 {
            Some(client) => {
                client.part(channel_name.clone());
                let _ = app_handle.emit("channel_part", channel_name.clone());
                Ok(channel_name)
            }
            None => Err(format!(
                "Failed to leave {}. No client configured.",
                channel_name
            )),
        }
    }
}
