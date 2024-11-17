use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};
use ts_rs::TS;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use crate::bot::BotData;
use crate::date::{
    date_time_is_greater_than_reference, get_date_time_minutes_ago, get_local_now, parse_date_time,
};

use super::{say, Bot, User};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Insults(Vec<Insult>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Insult {
    id: String,
    value: String,
}

pub async fn insult_thread_loop(app_handle: AppHandle, rx: Receiver<()>) {
    println!("Starting new insult thread!");
    loop {
        println!("Looping insult thread.");
        let bot_state = app_handle.state::<Bot>();
        let (enable_insults, time_between_insults) = {
            let bot_info = bot_state
                .bot_info
                .lock()
                .expect("Failed to get lock for bot_info");

            (bot_info.enable_insults, bot_info.time_between_insults)
        };

        let sleep_time = Duration::from_secs(time_between_insults as u64);
        thread::sleep(sleep_time);

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Shutting down insult thread.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        if enable_insults {
            let bot_data_state = app_handle.state::<BotData>();

            let insults = {
                let insults = bot_data_state
                    .insults
                    .lock()
                    .expect("Failed to get lock for insults.");

                insults.0.clone()
            };

            // Pick a random insult.
            let random_insult = insults.choose(&mut rand::thread_rng());

            match random_insult {
                Some(insult) => {
                    let mut formatted_message = insult.value.clone();

                    if formatted_message.contains("{{streamer}}") {
                        let channel_name = {
                            let state = app_handle.state::<Bot>();
                            let bot_info = state.bot_info.lock().expect("Failed to get bot_info");
                            bot_info.channel_name.clone()
                        };

                        formatted_message =
                            formatted_message.replace("{{streamer}}", channel_name.as_str())
                    }

                    if formatted_message.contains("{{user}}") {
                        let random_user = get_random_user(
                            app_handle.clone(),
                            !insult.value.contains("{{streamer}}"),
                        );

                        match random_user {
                            Some(user) => {
                                formatted_message =
                                    formatted_message.replace("{{user}}", user.username.as_str());
                            }
                            None => {
                                // Could not successfully get a random user to insult.
                                println!("Could not find a random user to insult.");
                                continue;
                            }
                        }
                    }

                    // Say it in chat.
                    let _ = say(bot_state.clone(), formatted_message.as_str()).await;
                }
                None => {
                    println!("Could not get a random insult.")
                }
            }
        }
    }
}

fn get_random_user(app_handle: AppHandle, streamer_inclusive: bool) -> Option<User> {
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

    use crate::bot::{BotData, Insults};
    use crate::file::{write_file, WriteFileError};

    use super::Insult;

    #[tauri::command]
    pub fn get_insults(app_handle: tauri::AppHandle) -> Vec<Insult> {
        let bot_data_state = app_handle.state::<BotData>();
        let insults = {
            bot_data_state
                .insults
                .lock()
                .expect("Failed to get lock for insults.")
                .0
                .clone()
        };

        insults
    }

    #[tauri::command]
    pub fn save_insults(app_handle: tauri::AppHandle, insults: Insults) -> Result<(), String> {
        let state = app_handle.state::<BotData>();
        *state
            .insults
            .lock()
            .expect("Failed to get lock for bot info") = insults.clone();

        let write_result = write_file::<Insults>(&app_handle, "insults.json", insults.clone());

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
            let _ = app_handle.emit("insults_update", insults);
        }

        Ok(())
    }

    #[tauri::command]
    pub fn delete_insult(app_handle: tauri::AppHandle, insult_id: String) -> Result<(), String> {
        let state = app_handle.state::<BotData>();
        let insults = {
            let mut insults = state
                .insults
                .lock()
                .expect("Failed to get lock for insults");

            match insults.0.iter().position(|insult| insult.id == insult_id) {
                None => return Err("Could not find index of insult.".to_string()),
                Some(index) => insults.0.remove(index),
            };

            insults.clone()
        };

        save_insults(app_handle.clone(), insults);

        Ok(())
    }
}
