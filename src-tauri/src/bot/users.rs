use std::collections::HashMap;

use tauri::{AppHandle, Emitter, Manager};
use ts_rs::TS;
use twitch_irc::message::TwitchUserBasics;

use super::BotData;
use crate::{date::get_local_now_formatted, file::write_file};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Users(pub HashMap<String, User>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct User {
    pub id: String,
    pub username: String,
    pub consented: bool,
    pub last_seen: String,
}

pub fn process_user_state(app_handle: AppHandle, user: &TwitchUserBasics) {
    // Get existing users.
    // Check if this user is already part of known users.
    // If not, assign default User values and save it to our files.
    // If they are known in our data, update any data we need to.

    let bot_data = app_handle.state::<BotData>();

    let mut users = bot_data
        .users
        .lock()
        .expect("Failed to get lock for users state.");

    match users.0.get_mut(&user.name) {
        // Create a new user
        None => {
            users.0.insert(
                user.name.clone(),
                User {
                    id: user.id.clone(),
                    username: user.name.clone(),
                    consented: false,
                    last_seen: get_local_now_formatted(),
                },
            );
        }
        // Update existing user
        Some(user) => {
            user.last_seen = get_local_now_formatted();
        }
    }

    if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
        println!("Failed to write users.json file to disk! {:?}", error);
        let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
    }
}
