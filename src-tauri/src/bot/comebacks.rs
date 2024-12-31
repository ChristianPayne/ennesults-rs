use rand::seq::SliceRandom;
use rand::Rng;
use tauri::{AppHandle, Manager};
use ts_rs::TS;
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{say, Bot, BotInfo};

use super::BotData;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Comebacks(pub Vec<Comeback>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Comeback {
    pub id: String,
    pub value: String,
}

pub async fn process_comebacks(app_handle: AppHandle, msg: &PrivmsgMessage) -> bool {
    let state = app_handle.state::<Bot>();

    let (bot_name, percent_chance_of_comeback, comeback_options, channel_name) = {
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        // Check to make sure comebacks are enabled in the settings.
        if !bot_info.enable_comebacks {
            return false;
        }

        let comeback_options = state
            .bot_data
            .comebacks
            .lock()
            .expect("Failed to get lock for bot data");

        if comeback_options.0.is_empty() {
            return false;
        }

        // Get bot name
        (
            bot_info.bot_name.clone(),
            bot_info.percent_chance_of_comeback,
            comeback_options.0.clone(),
            bot_info.channel_name.clone(),
        )
    };

    // Check if bot name is in msg.
    if (msg
        .message_text
        .to_lowercase()
        .contains(bot_name.to_lowercase().as_str())
        || msg.message_text.to_lowercase().contains("ennegisults"))
        && rand::thread_rng().gen_ratio(percent_chance_of_comeback, 100)
    {
        let mut random_comeback = comeback_options.choose(&mut rand::thread_rng());

        if let Some(comeback) = random_comeback {
            let mut formatted_comeback = comeback.value.clone();

            formatted_comeback = formatted_comeback.replace("{{user}}", msg.sender.name.as_str());
            formatted_comeback = formatted_comeback.replace("{{streamer}}", channel_name.as_str());

            let _ = say(state.clone(), formatted_comeback.as_str()).await;
            return true;
        }
    }

    false
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::{Bot, BotData, Comebacks};
    use crate::file::{write_file, WriteFileError};

    use super::Comeback;

    #[tauri::command]
    pub fn get_comebacks(app_handle: tauri::AppHandle) -> Vec<Comeback> {
        let state = app_handle.state::<Bot>();
        let comebacks = {
            state
                .bot_data
                .comebacks
                .lock()
                .expect("Failed to get lock for comebacks.")
                .0
                .clone()
        };

        comebacks
    }

    #[tauri::command]
    pub fn get_comebacks_count(app_handle: tauri::AppHandle) -> u32 {
        let state = app_handle.state::<Bot>();
        let comebacks = state
            .bot_data
            .comebacks
            .lock()
            .expect("Failed to get lock for insults.");

        comebacks.0.len() as u32
    }

    #[tauri::command]
    pub fn update_comeback(app_handle: tauri::AppHandle, comeback: Comeback) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut comebacks = state
            .bot_data
            .comebacks
            .lock()
            .expect("Failed to get lock for insults.")
            .clone();

        match comebacks.0.iter_mut().find(|i| i.id == comeback.id) {
            Some(comeback_in_db) => {
                comeback_in_db.value = comeback.value;
            }
            None => {
                return Err("Failed to find insult in database.".to_string());
            }
        }
        save_comebacks(app_handle, comebacks)?;

        Ok(())
    }

    #[tauri::command]
    pub fn save_comebacks(
        app_handle: tauri::AppHandle,
        comebacks: Comebacks,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        *state
            .bot_data
            .comebacks
            .lock()
            .expect("Failed to get lock for bot info") = comebacks.clone();

        let write_result =
            write_file::<Comebacks>(&app_handle, "comebacks.json", comebacks.clone());

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
            let _ = app_handle.emit("comebacks_update", comebacks);
        }

        Ok(())
    }

    #[tauri::command]
    pub fn delete_comeback(
        app_handle: tauri::AppHandle,
        comeback_id: String,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let comebacks = {
            let mut comebacks = state
                .bot_data
                .comebacks
                .lock()
                .expect("Failed to get lock for comebacks");

            match comebacks
                .0
                .iter()
                .position(|comeback| comeback.id == comeback_id)
            {
                None => return Err("Could not find index of comeback.".to_string()),
                Some(index) => comebacks.0.remove(index),
            };

            comebacks.clone()
        };

        save_comebacks(app_handle.clone(), comebacks);

        Ok(())
    }
}
