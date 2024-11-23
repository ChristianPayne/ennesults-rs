use std::collections::HashMap;

use tauri::{AppHandle, Emitter, Manager};
use ts_rs::TS;
use twitch_irc::message::TwitchUserBasics;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use crate::date::{
    date_time_is_greater_than_reference, get_date_time_minutes_ago, get_local_now, parse_date_time,
};

use super::{Bot, BotData};
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

    tokio::spawn(emit_active_users(app_handle.clone()));

    if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
        println!("Failed to write users.json file to disk! {:?}", error);
        let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
    } else {
        let _ = app_handle.emit(
            "users_update",
            users.0.clone().into_values().collect::<Vec<User>>(),
        );
    }
}

async fn emit_active_users(app_handle: AppHandle) {
    let bot_data_state = app_handle.state::<BotData>();
    if let Ok(active_users) = api::get_active_users(bot_data_state).await {
        let _ = app_handle.emit("active_users", active_users);
    }
}

pub fn get_random_user(app_handle: AppHandle, streamer_inclusive: bool) -> Option<User> {
    let bot_data_state = app_handle.state::<BotData>();
    let users = bot_data_state
        .users
        .lock()
        .expect("Failed to get lock for users.");

    let bot_state = app_handle.state::<Bot>();
    let bot_info = bot_state
        .bot_info
        .lock()
        .expect("Failed to get lock for bot info");

    users
        .0
        .clone()
        .into_values()
        .filter(|user| {
            // If it is the streamer, check if we want to include them.
            if user.username == bot_info.channel_name {
                return streamer_inclusive;
            }
            // Check lurk status of all users.
            let user_is_not_lurking = match parse_date_time(user.last_seen.as_str()) {
                // If we error on parsing the last seen, let's not include the user as an option.
                Err(_) => false,
                // Calculate if the user's last seen date is within the lurk timer.
                Ok(user_last_seen) => {
                    let time_min_ago = get_date_time_minutes_ago(bot_info.lurk_time);
                    date_time_is_greater_than_reference(time_min_ago, user_last_seen.into())
                }
            };

            user_is_not_lurking && user.consented
        })
        .choose(&mut rand::thread_rng())
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::{
        bot::{BotData, User},
        file::write_file,
    };

    #[tauri::command]
    pub async fn get_users(state: tauri::State<'_, BotData>) -> Result<Vec<User>, String> {
        let users = state
            .users
            .lock()
            .expect("Failed to get lock for users state.");

        Ok(users.0.clone().into_values().collect())
    }

    #[tauri::command]
    pub async fn get_active_users(state: tauri::State<'_, BotData>) -> Result<(u32, u32), String> {
        let users = state
            .users
            .lock()
            .expect("Failed to get lock for users state.");

        let total_users = users.0.len() as u32;
        let active_users = users.0.values().filter(|u| u.consented).map(|_| 1).sum();

        Ok((total_users, active_users))
    }

    #[tauri::command]
    pub async fn delete_user(
        app_handle: tauri::AppHandle,
        username: String,
    ) -> Result<String, String> {
        let state = app_handle.state::<BotData>();
        let mut users = state
            .users
            .lock()
            .expect("Failed to get lock for users state.");

        let _ = users.0.remove(&username);

        if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
            println!("Failed to write users.json file to disk! {:?}", error);
            let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
        } else {
            let _ = app_handle.emit(
                "users_update",
                users.0.clone().into_values().collect::<Vec<User>>(),
            );
        }

        Ok(username)
    }
}
