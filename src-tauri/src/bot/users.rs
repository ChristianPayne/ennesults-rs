use std::collections::HashMap;

use tauri::{AppHandle, Manager};
use ts_rs::TS;
use twitch_irc::message::TwitchUserBasics;

use super::BotData;

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
}
