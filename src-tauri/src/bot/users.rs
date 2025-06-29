// use serde_partial::SerializePartial;
use std::collections::HashMap;
use tauri::{AppHandle, Emitter, Manager};
use ts_rs::TS;
use twitch_irc::message::TwitchUserBasics;

use rand::seq::IteratorRandom;

use crate::helpers::date::{
    date_time_is_greater_than_reference, get_date_time_minutes_ago, get_local_now_formatted,
    parse_date_time,
};

use super::Bot;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct Users(pub HashMap<String, User>);

impl Users {
    pub fn from(users: Vec<User>) -> Self {
        let mut users_hash: HashMap<String, User> = HashMap::new();

        for user in users {
            users_hash.insert(user.username.clone(), user);
        }

        Self(users_hash)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct User {
    pub id: String,
    pub username: String,
    pub consented: bool,
    pub last_seen: String,
    #[serde(default = "default_bool")]
    pub lurk: bool,
}

fn default_bool() -> bool {
    false
}

pub fn process_user_state(app_handle: AppHandle, user: &TwitchUserBasics) {
    let state = app_handle.state::<Bot>();
    let mut users = state.bot_data.get_users();

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
                    lurk: false,
                },
            );
        }
        // Update existing user
        Some(user) => {
            user.lurk = false;
            user.last_seen = get_local_now_formatted();
        }
    }

    tokio::spawn(emit_active_users(app_handle.clone()));

    let _ = state.bot_data.save_users(app_handle.clone(), &users);
}

async fn emit_active_users(app_handle: AppHandle) {
    let state = app_handle.state::<Bot>();
    if let Ok(active_users) = api::get_active_users(state).await {
        let _ = app_handle.emit("active_users", active_users);
    }
}

pub fn get_random_user(
    app_handle: AppHandle,
    streamer_inclusive: bool,
    users: &Users,
    user_must_be_consented: bool,
) -> Option<&User> {
    let bot_state = app_handle.state::<Bot>();
    let settings = bot_state
        .settings
        .lock()
        .expect("Failed to get lock for settings");

    users
        .0
        .values()
        .filter(|user| {
            // If it is the streamer, check if we want to include them.
            if user.username == settings.channel_name {
                return streamer_inclusive;
            }
            // Check lurk status of all users.
            let user_has_spoken_since_lurk_timer = match parse_date_time(user.last_seen.as_str()) {
                // If we error on parsing the last seen, let's not include the user as an option.
                Err(_) => false,
                // Calculate if the user's last seen date is within the lurk timer.
                Ok(user_last_seen) => {
                    let time_min_ago = get_date_time_minutes_ago(settings.lurk_time);
                    date_time_is_greater_than_reference(time_min_ago, user_last_seen.into())
                }
            };

            !user.lurk && user_has_spoken_since_lurk_timer && {
                if user_must_be_consented {
                    user.consented
                } else {
                    true
                }
            }
        })
        .choose(&mut rand::thread_rng())
}

pub mod api {
    use tauri::Manager;

    use crate::bot::{users::User, Bot};

    #[tauri::command]
    pub async fn get_users(state: tauri::State<'_, Bot>) -> Result<Vec<User>, String> {
        let users = state
            .bot_data
            .users
            .lock()
            .expect("Failed to get lock for users state.");

        Ok(users.0.clone().into_values().collect())
    }

    #[tauri::command]
    pub async fn get_active_users(state: tauri::State<'_, Bot>) -> Result<(u32, u32), String> {
        let users = state
            .bot_data
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
        let state = app_handle.state::<Bot>();

        let mut users = state.bot_data.get_users();
        let _ = users.0.remove(&username);

        let _ = state.bot_data.save_users(app_handle.clone(), &users);

        Ok(username)
    }
}
