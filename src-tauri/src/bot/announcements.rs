use rand::seq::SliceRandom;
use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
    time::Duration,
};
use tauri::{AppHandle, Manager};
use tokio::task::JoinHandle;
use ts_rs::TS;

use super::{get_random_user, say, Bot, User, Users};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Announcements(pub Vec<Announcement>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Announcement {
    pub id: String,
    pub value: String,
}

pub fn run_announcement(app_handle: AppHandle) -> Option<String> {
    let mut next_announcement_index: usize = 0;

    let state = app_handle.state::<Bot>();
    let randomize_announcements = {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings");

        settings.randomize_announcements
    };

    let state = app_handle.state::<Bot>();

    let announcements = {
        let announcements = state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for insults.");

        announcements.0.clone()
    };

    // Pick a random announcement.
    let announcement = {
        if randomize_announcements {
            announcements.choose(&mut rand::thread_rng())
        } else {
            // Next announcement
            if announcements.is_empty() {
                None
            } else {
                let x = &announcements[next_announcement_index];

                next_announcement_index += 1;

                if next_announcement_index == announcements.len() {
                    next_announcement_index = 0
                }

                Some(x)
            }
        }
    };

    match announcement {
        Some(announcement) => format_announcement(app_handle.clone(), announcement, None),
        None => {
            println!("Could not get an announcement to say.");
            None
        }
    }
}

pub fn format_announcement(
    app_handle: tauri::AppHandle,
    announcement: &Announcement,
    user_pool: Option<Vec<User>>,
) -> Option<String> {
    let state = app_handle.state::<Bot>();
    let mut formatted_message = announcement.value.clone();

    // Format for any streamer tags.
    if formatted_message.contains("{{streamer}}") {
        let channel_name = {
            let state = app_handle.state::<Bot>();
            state.get_channel_name()
        };

        formatted_message = formatted_message.replace("{{streamer}}", channel_name.as_str())
    }

    // Format for any version tags.
    if formatted_message.contains("{{version}}") {
        let version = format!("v{}", app_handle.package_info().version.clone());

        formatted_message = formatted_message.replace("{{version}}", &version)
    }

    if formatted_message.contains("{{random}}") {
        let mut users: Users = {
            match user_pool {
                None => state.bot_data.get_users(),
                Some(users) => Users::from(users),
            }
        };

        // Format for any random tags.
        while formatted_message.contains("{{random}}") {
            let random_user = get_random_user(
                app_handle.clone(),
                !announcement.value.contains("{{streamer}}"),
                &users,
                true,
            )
            .cloned();

            match random_user {
                Some(user) => {
                    // Remove the user so that we don't pick it again if we go around again.
                    users.0.remove(&user.username);

                    // Replace just the first instance of the tag.
                    formatted_message =
                        formatted_message.replacen("{{random}}", user.username.as_str(), 1);
                }
                None => {
                    println!(
                        "🟡 Not enough random consented users available to format announcement."
                    );
                    return None;
                }
            }
        }
    }

    Some(formatted_message)
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::Bot;
    use crate::helpers::file::{write_file, WriteFileError};

    use super::{Announcement, Announcements};

    #[tauri::command]
    pub fn get_announcements(app_handle: tauri::AppHandle) -> Vec<Announcement> {
        let state = app_handle.state::<Bot>();
        let announcements = {
            state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements.")
                .0
                .clone()
        };

        announcements
    }

    #[tauri::command]
    pub fn update_announcement(
        app_handle: tauri::AppHandle,
        announcement: Announcement,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut announcements = state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for announcements.")
            .clone();

        match announcements.0.iter_mut().find(|i| i.id == announcement.id) {
            Some(announcement_in_db) => {
                announcement_in_db.value = announcement.value;
            }
            None => return Err("Failed to find announcement in database".to_string()),
        }

        save_announcements(app_handle, announcements)?;

        Ok(())
    }

    #[tauri::command]
    pub fn save_announcements(
        app_handle: tauri::AppHandle,
        announcements: Announcements,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        *state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for settings") = announcements.clone();

        let write_result =
            write_file::<Announcements>(&app_handle, "announcements.json", announcements.clone());

        if let Some(err) = write_result.err() {
            match err {
                WriteFileError::FailedConvertJSON => {
                    return Err("Failed to convert to json.".to_string())
                }
                WriteFileError::FailedCreateFile => {
                    return Err("Failed to create file.".to_string())
                }
                WriteFileError::FailedWriteFile => {
                    return Err("Failed to write contents in file.".to_string())
                }
            }
        } else {
            let _ = app_handle.emit("announcements_update", announcements);
        }

        Ok(())
    }

    #[tauri::command]
    pub fn delete_announcement(
        app_handle: tauri::AppHandle,
        announcement_id: String,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let announcements = {
            let mut announcements = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements");
            match announcements
                .0
                .iter()
                .position(|announcement| announcement.id == announcement_id)
            {
                None => return Err("Could not find index of insult.".to_string()),
                Some(index) => announcements.0.remove(index),
            };

            announcements.clone()
        };

        let _ = save_announcements(app_handle.clone(), announcements);

        Ok(())
    }
}
