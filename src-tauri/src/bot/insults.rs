use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};
use ts_rs::TS;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use crate::bot::BotData;

use super::{say, Bot};

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
            let mut random_insult = insults.choose(&mut rand::thread_rng());

            if let Some(insult) = random_insult.take() {
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
                    let mut random_user = {
                        let state = app_handle.state::<BotData>();
                        let users = state.users.lock().expect("Failed to get lock for users.");

                        users
                            .0
                            .clone()
                            .into_values()
                            .filter(|user| user.consented)
                            .choose(&mut rand::thread_rng())
                    };

                    if let Some(user) = random_user.take() {
                        formatted_message =
                            formatted_message.replace("{{user}}", user.username.as_str())
                    }
                }

                // Say it in chat.
                let _ = say(bot_state.clone(), formatted_message.as_str()).await;
            }
        }
    }
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
