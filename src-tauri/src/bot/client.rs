use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task::JoinHandle;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::{ServerMessage, UserNoticeEvent};
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use super::{
    handle_whisper, process_comebacks, process_corrections, process_user_state, Bot, MessageThread,
    SerializeRBGColor, TwitchMessage,
};
use crate::commands::{meets_minimum_user_level, parse_for_command, parse_msg_for_user_level};

// CLIENT
#[derive(Debug, Default)]
pub enum Client {
    Connected {
        client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        client_join_handle: JoinHandle<()>,
        message_thread: MessageThread,
    },
    #[default]
    Disconnected,
}

impl Client {
    pub fn new(
        twitch_client: TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>,
        twitch_client_thread_handle: JoinHandle<()>,
        message_thread: MessageThread,
    ) -> Self {
        Client::Connected {
            client: twitch_client,
            client_join_handle: twitch_client_thread_handle,
            message_thread,
        }
    }

    pub fn get_client(&self) -> Option<TwitchIRCClient<TCPTransport<TLS>, StaticLoginCredentials>> {
        match self {
            Client::Disconnected => None,
            Client::Connected { client, .. } => Some(client.clone()),
        }
    }

    pub async fn queue_message(&self, message: String) {
        match self {
            Client::Connected { message_thread, .. } => {
                message_thread.queue_message(message).await;
            }
            Client::Disconnected => (),
        }
    }
}

#[tauri::command]
pub async fn say(state: tauri::State<'_, Bot>, message: &str) -> Result<(), String> {
    let channel_name = {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings.");

        if settings.channel_name.is_empty() {
            return Err("Channel name not found.".into());
        }
        settings.channel_name.clone()
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

    if !channel_joined {
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

                    app_handle
                        .emit("message", twitch_message)
                        .expect("Failed to emit twitch message.");
                }

                // Always process user state first so we keep track of the last seen time.
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
                    println!("🤖 Comeback complete!");
                } else if process_corrections(app_handle.clone(), &msg).await {
                    // Should we do something?
                    println!("🤖 Correction complete!");
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
                    profile_image_url: _,
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
            handle_incoming_chat, validate_auth, Authentication, AuthenticationError, Bot,
            ChannelDetails, MessageThread,
        },
        twitch::get_broadcaster_id,
    };
    use std::ops::Deref;
    use tauri::{AppHandle, Emitter, Manager};
    use twitch_irc::{
        login::StaticLoginCredentials, ClientConfig, SecureTCPTransport, TwitchIRCClient,
    };

    use super::Client;

    #[tauri::command]
    pub async fn connect_to_twitch(app_handle: AppHandle) -> Result<Authentication, String> {
        let state = app_handle.state::<Bot>();
        // Handle the disconnecting of existing client connections to Twitch and any threads that are currently running.
        let _ = disconnect_from_twitch(app_handle.clone());

        println!("🤖 Connecting to Twitch...");
        let existing_auth = {
            let auth = state.auth.lock().expect("Failed to get lock for auth");
            auth.clone()
        };

        // dbg!(&existing_auth);

        let details = match existing_auth {
            Authentication::Valid { details, .. } => details,
            Authentication::Invalid { reason: _ } => {
                println!("❌ Failed to connect to Twitch. Auth invalid.");
                return Err("Authentication was not valid when connecting to Twitch.".to_string());
            }
            Authentication::NotSignedIn => {
                println!("❌ Failed to connect to Twitch. Not signed in.");
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
                AuthenticationError::ParsingError(message) => {
                    println!("❌ Authentication Error. Auth invalid. {}", &message);
                    message.clone()
                }
            })?;

        // Save our new valid authentication
        {
            let mut auth = state.auth.lock().expect("Failed to get lock for auth");
            *auth = authentication.clone();
            let _ = app_handle.emit("auth", authentication.clone());
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
                let err = format!("Failed to authenticate bot. {}", reason);
                dbg!(&err);
                return Err(err);
            }
            Authentication::NotSignedIn => {
                let err = "Failed to authenticate bot. Not signed in.";
                dbg!(&err);
                return Err(err.to_string());
            }
        };

        let (incoming_messages, twitch_client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // First thing we should do is start consuming incoming messages, otherwise they will back up.
        let twitch_client_thread_handle =
            tokio::spawn(handle_incoming_chat(app_handle.clone(), incoming_messages));

        let message_thread = MessageThread::new(app_handle.clone());

        let mut client = state.client.lock().expect("Failed to get lock for client");
        *client = Client::new(twitch_client, twitch_client_thread_handle, message_thread);

        println!("✅ Connected to Twitch!");

        Ok(authentication.clone())
    }

    #[tauri::command]
    pub fn disconnect_from_twitch(app_handle: AppHandle) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut client = state.client.lock().expect("Failed to get lock for client");
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings");

        match &mut *client {
            Client::Disconnected => Err("Client already disconnected".to_string()),
            Client::Connected {
                client: twitch_client,
                client_join_handle,
                message_thread,
            } => {
                // Shut down the message thread if it is running.
                let _ = message_thread.shutdown();

                // Tell the client to leave the twitch channel.
                twitch_client.part(settings.channel_name.clone());
                let _ = app_handle.emit("channel_part", settings.channel_name.clone());

                client_join_handle.abort();

                // Update the state to reflect the client being disconnected.
                *client = Client::Disconnected;

                Ok(())
            }
        }
    }

    #[tauri::command]
    pub async fn connect_to_channel(app_handle: AppHandle) -> Result<String, String> {
        let state = app_handle.state::<Bot>();
        let channel_name = {
            state
                .settings
                .lock()
                .expect("Failed to get lock for settings")
                .channel_name
                .clone()
        };

        if channel_name.is_empty() {
            return Err("Channel name not found.".into());
        }

        let mut authentication = {
            state
                .auth
                .lock()
                .expect("Failed to get lock for auth")
                .clone()
        };

        // The idea here is that we want to alter the authentication to hold onto a connection status of the channel. That way we don't have to keep track of multiple things in different places.

        let result = match &mut authentication {
            Authentication::Valid { details, .. } => {
                let broadcaster_id = get_broadcaster_id(
                    details.client_id.clone(),
                    details.access_token.clone(),
                    channel_name.clone(),
                )
                .await?;

                let Some(client) = state.client.lock().unwrap().get_client() else {
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
                                details.set_channel_details(ChannelDetails::Connected {
                                    channel_id: broadcaster_id,
                                });
                                Ok(channel_name.clone())
                            }
                            Err(e) => Err(format!("Could not join channel! {}", e)),
                        }
                    }
                }
            }
            Authentication::NotSignedIn | Authentication::Invalid { .. } => {
                Err("Authorization not valid. Can't connect to channel.".to_string())
            }
        };

        // save authentication back to state
        let mut auth = state
            .auth
            .lock()
            .expect("Failed to get authentication lock.");

        *auth = authentication;

        result
    }

    #[tauri::command]
    pub async fn get_channel_status(state: tauri::State<'_, Bot>) -> Result<(bool, bool), String> {
        let channel_name = state
            .settings
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
            .settings
            .lock()
            .expect("Failed to get lock")
            .channel_name
            .clone();
        let client = state.client.lock().unwrap();
        match client.deref() {
            Client::Disconnected => Ok("No client connected.".to_string()),
            Client::Connected { client, .. } => {
                client.part(channel_name.clone());
                let _ = app_handle.emit("channel_part", channel_name.clone());
                Ok(channel_name)
            }
        }
    }
}
