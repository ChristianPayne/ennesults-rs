use api::get_channel_status;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{ServerMessage, UserNoticeEvent};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};

use super::{
    handle_whisper, process_comebacks, process_corrections, process_user_state, AnnouncementThread,
    Bot, BotData, InsultThread, SerializeRBGColor, TwitchMessage,
};
use crate::bot::api::get_bot_info;
use crate::bot::Authentication;
use crate::commands::{meets_minimum_user_level, parse_for_command, parse_msg_for_user_level};
use crate::twitch::get_broadcaster_id;

use serde_json::Value;

// CLIENT
#[derive(Debug, Default)]
pub enum Client {
    Connected {
        client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        client_join_handle: JoinHandle<()>,
        insult_thread: InsultThread,
        announcement_thread: AnnouncementThread,
    },
    #[default]
    Disconnected,
}

impl Client {
    pub fn new(
        twitch_client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        twitch_client_thread_handle: JoinHandle<()>,
        insult_thread: InsultThread,
        announcement_thread: AnnouncementThread,
    ) -> Self {
        Client::Connected {
            client: twitch_client,
            client_join_handle: twitch_client_thread_handle,
            insult_thread,
            announcement_thread,
        }
    }
    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        match self {
            Client::Disconnected => None,
            Client::Connected {
                client,
                client_join_handle,
                insult_thread,
                announcement_thread,
            } => Some(client.clone()),
        }
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

    let (_, channel_joined) = client.get_channel_status(channel_name.clone()).await;

    if (!channel_joined) {
        return Err("No channel joined".to_string());
    }

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
                    // dbg!(&msg);
                    let mut chat_messages = bot
                        .chat_messages
                        .lock()
                        .expect("Failed to get lock for chat_messages on bot state.");

                    let twitch_message = TwitchMessage {
                        message_id: msg.message_id.clone(),
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

                // Chained if else statements so we only do one of the options.
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
                } else if process_comebacks(app_handle.clone(), &msg).await {
                    // Should we do something?
                } else if process_corrections(app_handle.clone(), &msg).await {
                    // Should we do something?
                }
            }
            ServerMessage::GlobalUserState(_) => (),
            ServerMessage::Ping(_) => (),
            ServerMessage::Pong(_) => (),
            ServerMessage::Join(msg) => {
                let _ = app_handle.emit("channel_join", msg.channel_login);
            }
            ServerMessage::Part(_) => (),
            ServerMessage::Generic(_) => (),
            ServerMessage::UserNotice(user_notice_message) => {
                if let UserNoticeEvent::Raid {
                    viewer_count,
                    profile_image_url,
                } = user_notice_message.event
                {
                    let raid_message = format!(
                        "{} raiding with {} viewers!",
                        user_notice_message.sender.name, viewer_count
                    );
                    // dbg!(&user_notice_message.channel_id);
                    let _ = say(app_handle.state::<Bot>(), &raid_message).await;
                } else {
                    dbg!(user_notice_message);
                }
            }
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
    use crate::{
        bot::{
            handle_incoming_chat, validate_auth, AnnouncementThread, Authentication,
            AuthenticationBuilder, AuthenticationDetails, AuthenticationError, Bot, InsultThread,
        },
        file::{write_file, WriteFileError},
    };
    use serde_json::{json, Value};
    use std::{collections::HashMap, ops::Deref};
    use tauri::{http, AppHandle, Emitter, Listener, Manager, Url};
    use twitch_irc::{
        login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
    };
    use url_builder::URLBuilder;

    use super::Client;

    #[tauri::command]
    pub async fn connect_to_twitch(app_handle: AppHandle) -> Result<Authentication, String> {
        let state = app_handle.state::<Bot>();
        // Handle the disconnecting of existing client connections to Twitch and any threads that are currently running.
        disconnect_from_twitch(app_handle.clone());

        println!("🤖 Connecting to Twitch...");
        let mut existing_auth = {
            let auth = state.auth.lock().expect("Failed to get lock for auth");
            auth.clone()
        };

        // dbg!(&existing_auth);

        let details = match existing_auth {
            Authentication::Valid { details, .. } => details,
            Authentication::Invalid { reason } => {
                return Err("Authentication was not valid when connecting to Twitch.".to_string());
            }
            Authentication::NotSignedIn => {
                return Err(
                    "Not signed into Twitch. Please connect your account in the settings page."
                        .to_string(),
                );
            }
        };

        // Validate authentication details.
        // Take our details and revalidate through Twitch. Validate every time.
        let authentication = validate_auth(app_handle.clone(), details.access_token.clone())
            .await
            .map_err(|e| match e {
                AuthenticationError::ParsingError(message) => message.clone(),
            })?;

        // Save our new valid authentication
        {
            let mut auth = state.auth.lock().expect("Failed to get lock for auth");
            *auth = authentication.clone();
            app_handle.emit("auth", authentication.clone());
        }

        let config = match &authentication {
            Authentication::Valid { details, .. } => {
                // Running auth validation when connecting to twitch.
                // Creating the config with the new auth.
                ClientConfig::new_simple(StaticLoginCredentials::new(
                    details.login.clone(),
                    Some(details.access_token.clone()),
                ))
            }
            Authentication::Invalid { reason } => {
                return Err(format!("Failed to authenticate bot. {}", reason))
            }
            Authentication::NotSignedIn => {
                return Err(format!("Failed to authenticate bot. Not signed in."))
            }
        };

        let (incoming_messages, twitch_client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // First thing we should do is start consuming incoming messages, otherwise they will back up.
        let twitch_client_thread_handle =
            tokio::spawn(handle_incoming_chat(app_handle.clone(), incoming_messages));

        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info");

        let insult_thread = InsultThread::new(app_handle.clone(), bot_info.enable_insults);

        let announcement_thread =
            AnnouncementThread::new(app_handle.clone(), bot_info.enable_announcements);

        let mut client = state.client.lock().expect("Failed to get lock for client");
        *client = Client::new(
            twitch_client,
            twitch_client_thread_handle,
            insult_thread,
            announcement_thread,
        );

        println!("✅ Connected to Twitch!");

        Ok(authentication.clone())
    }

    #[tauri::command]
    pub fn disconnect_from_twitch(app_handle: AppHandle) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut client = state.client.lock().expect("Failed to get lock for client");
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info");

        match &mut *client {
            Client::Disconnected => Err("Client already disconnected".to_string()),
            Client::Connected {
                client: twitch_client,
                client_join_handle,
                insult_thread,
                announcement_thread,
            } => {
                // Shut down the insult thread if it is running.
                if let InsultThread::Running {
                    handle: insult_handle,
                    sender: insult_sender,
                } = insult_thread
                {
                    insult_sender.send(());
                    insult_handle.abort();
                }

                // Shut down the announcement thread if it is running.
                if let AnnouncementThread::Running {
                    handle: announcement_handle,
                    sender: announcement_sender,
                } = announcement_thread
                {
                    announcement_sender.send(());
                    announcement_handle.abort();
                }

                // Tell the client to leave the twitch channel.
                twitch_client.part(bot_info.channel_name.clone());
                let _ = app_handle.emit("channel_part", bot_info.channel_name.clone());

                // Update the state to reflect the client being disconnected.
                *client = Client::Disconnected;

                Ok(())
            }
        }
    }

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
                    Ok(_) => {
                        println!("✅ Connected to {}!", channel_name.clone());
                        Ok(channel_name.clone())
                    }
                    Err(e) => Err(format!("Could not join channel! {}", e)),
                }
            }
        }
    }

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

        let Some(client) = state.client.lock().unwrap().get_client() else {
            return Err("Can't get channel status. Not connected to Twitch.".into());
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
        match client.deref() {
            Client::Disconnected => Err(format!(
                "Failed to leave {}. No client connected.",
                channel_name
            )),
            Client::Connected {
                client,
                client_join_handle,
                insult_thread,
                announcement_thread,
            } => {
                client.part(channel_name.clone());
                let _ = app_handle.emit("channel_part", channel_name.clone());
                Ok(channel_name)
            }
        }
    }
}
