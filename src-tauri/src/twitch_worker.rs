use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc::UnboundedReceiver;
use twitch_irc::message::ServerMessage;

use crate::bot::{Bot, BotData, SerializeRBGColor, TwitchMessage, User};

pub async fn process_twitch_messages(app_handle: AppHandle, mut incoming_messages: UnboundedReceiver<ServerMessage>) {
  while let Some(message) = incoming_messages.recv().await {
    match message {
        ServerMessage::Privmsg(msg) => {
            // println!("Received message: {:?}", msg);
            let bot = app_handle.state::<Bot>();

            let mut chat_messages = bot.chat_messages.lock().expect("Failed to get lock for chat_messages on bot state.");
            
            let twitch_message = TwitchMessage {
                username: msg.sender.name,
                message: msg.message_text,
                color: msg.name_color.map(|color| SerializeRBGColor(color.r, color.g, color.b))
            };

            chat_messages.push(twitch_message.clone());

            app_handle.emit("message", twitch_message).unwrap();
        },
        ServerMessage::GlobalUserState(user) => {
          let bot_data = app_handle.state::<BotData>();
          let mut users = bot_data.users.lock().unwrap();
          users.0.push(User {
            id: user.user_id,
            username: user.user_name,
            consented: false,
          });
          println!("Adding new user: {:?}", users);
        },
        ServerMessage::Pong(_) => {},
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
}