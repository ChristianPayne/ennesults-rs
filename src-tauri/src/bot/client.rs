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
use crate::bot::AuthValidation;
use crate::commands::{meets_minimum_user_level, parse_for_command, parse_msg_for_user_level};

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

    pub fn connect_to_twitch(&mut self, app_handle: tauri::AppHandle) -> Result<(), String> {
        println!("Connecting to Twitch...");
        let state = app_handle.state::<Bot>();
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        // Handle the disconnecting of existing client connections to Twitch and any threads that are currently running.
        self.disconnect_from_twitch(app_handle.clone());

        let auth_guard = state.auth.lock().expect("Failed to get Auth");
        let config = match auth_guard.clone() {
            AuthValidation::Valid {
                access_token,
                login,
                expires_in,
            } => ClientConfig::new_simple(StaticLoginCredentials::new(login, Some(access_token))),
            _ => return Err("Failed to authenticate bot. Auth not valid.".to_string()),
        };

        let (incoming_messages, twitch_client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

        // First thing we should do is start consuming incoming messages, otherwise they will back up.
        let twitch_client_thread_handle =
            tokio::spawn(handle_incoming_chat(app_handle.clone(), incoming_messages));

        let insult_thread = InsultThread::new(app_handle.clone(), bot_info.enable_insults);

        let announcement_thread =
            AnnouncementThread::new(app_handle.clone(), bot_info.enable_announcements);

        *self = Client::new(
            twitch_client,
            twitch_client_thread_handle,
            insult_thread,
            announcement_thread,
        );

        Ok(())
    }

    pub fn disconnect_from_twitch(&mut self, app_handle: tauri::AppHandle) {
        let bot_info = get_bot_info(app_handle.state::<Bot>());

        match self {
            Client::Disconnected => {}
            Client::Connected {
                client,
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
                client.part(bot_info.channel_name.clone());

                // Update the state to reflect the client being disconnected.
                *self = Client::Disconnected;
            }
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

pub async fn validate_auth(access_token: String) -> Result<AuthValidation, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", access_token))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    let resp: Value = serde_json::from_str(&resp).map_err(|e| e.to_string())?;

    match (
        resp["login"].clone(),
        resp["expires_in"].clone(),
        resp["message"].clone(),
    ) {
        (Value::String(login), Value::Number(expires_in), _) => {
            if let Some(expires_in) = expires_in.as_i64() {
                Ok(AuthValidation::Valid {
                    access_token,
                    expires_in,
                    login,
                })
            } else {
                Err("Failed to convert expires_in value".to_string())
            }
        }
        (Value::Null, Value::Null, Value::String(message)) => {
            Ok(AuthValidation::Invalid { reason: message })
        }
        _ => Err("Failed to parse response contents".to_string()),
    }
}

pub mod api {
    use crate::{
        bot::{AuthValidation, Bot},
        file::{write_file, WriteFileError},
    };
    use serde_json::{json, Value};
    use std::{collections::HashMap, ops::Deref};
    use tauri::{http, AppHandle, Emitter, Listener, Manager, Url};
    use url_builder::URLBuilder;

    use super::{validate_auth, Client};

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

    #[tauri::command]
    pub fn open_auth_window(app_handle: AppHandle) {
        let mut ub = URLBuilder::new();
        ub.set_protocol("https")
            .set_host("id.twitch.tv/oauth2/authorize")
            .add_param("response_type", "token")
            .add_param("client_id", "nbdppbmm4iicute0sl1cj663xyvbi4")
            .add_param("redirect_uri", format!("http://localhost:{}", 4500).as_str())
            .add_param("scope", "channel:bot moderator:read:chatters moderator:read:followers moderator:read:shoutouts moderator:manage:shoutouts chat:read whispers:read user:write:chat chat:edit".replace(":", "%3A").replace(" ", "%20").as_str())
            .add_param("state", "ennesults-rocks");

        let url = ub.build();

        let webview_url = tauri::WebviewUrl::App(url.into());
        // First window
        let window_result =
            tauri::WebviewWindowBuilder::new(&app_handle, "auth", webview_url.clone())
                .title("Ennesults Authentication")
                .incognito(true)
                .build();

        if let Ok(window) = window_result {}
    }

    #[tauri::command]
    pub async fn decode_auth_redirect(
        app_handle: AppHandle,
        url: String,
    ) -> Result<AuthValidation, String> {
        let url = url.replace("#", "?");
        let parsed_url = Url::parse(&url).unwrap();
        let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();

        // If we got an access token back, let's save it.
        match hash_query.get("access_token") {
            None => {
                // Send an emit to the front end that we didn't get the access token.
                let _ = app_handle.emit("error", "Failed to decode access token!");
                Err("Failed to decode access token!".to_string())
            }
            Some(access_token) => {
                // Save the access token.
                println!("Successfully received access token: {}", access_token);

                // Do a second query to check to make sure we have the bot name.
                let auth_info = validate_auth(access_token.clone()).await?;

                // Save auth info here.
                let write_result =
                    write_file::<AuthValidation>(&app_handle, "auth.json", auth_info.clone());

                if let Some(err) = write_result.err() {
                    return match err {
                        WriteFileError::FailedConvertJSON => {
                            Err("Failed to convert to json.".to_string())
                        }
                        WriteFileError::FailedCreateFile => {
                            Err("Failed to create file.".to_string())
                        }
                        WriteFileError::FailedWriteFile => {
                            Err("Failed to write contents in file.".to_string())
                        }
                    };
                }

                let bot = app_handle.state::<Bot>();
                let mut auth = bot.auth.lock().expect("Failed to get lock for Auth");
                *auth = auth_info.clone();
                Ok(auth_info)
            }
        }
    }

    #[tauri::command]
    pub fn sign_out_of_twitch(app_handle: AppHandle) -> Result<AuthValidation, String> {
        let bot = app_handle.state::<Bot>();

        let write_result = write_file::<Value>(&app_handle, "auth.json", Value::Null);

        if let Some(err) = write_result.err() {
            return match err {
                WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
                WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
                WriteFileError::FailedWriteFile => {
                    Err("Failed to write contents in file.".to_string())
                }
            };
        }

        {
            // Disconnect from Twitch.
            let mut client = bot.client.lock().expect("Failed to get lock for client");
            client.disconnect_from_twitch(app_handle.clone());
        }

        {
            let mut auth = bot.auth.lock().expect("Failed to get lock for auth");
            *auth = AuthValidation::NotSignedIn;
        }

        Ok(AuthValidation::NotSignedIn)
    }
}
