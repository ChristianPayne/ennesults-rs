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
    pub id: u16,
    pub value: String,
}

pub async fn process_comebacks(app_handle: AppHandle, msg: &PrivmsgMessage) -> bool {
    let bot_state = app_handle.state::<Bot>();
    let bot_data_state = app_handle.state::<BotData>();

    let (bot_name, percent_chance_of_comeback, comeback_options) = {
        let bot_info = bot_state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        // Check to make sure comebacks are enabled in the settings.
        if !bot_info.enable_comebacks {
            return false;
        }

        let comeback_options = bot_data_state
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
        )
    };

    // Check if bot name is in msg.
    if msg
        .message_text
        .to_lowercase()
        .contains(bot_name.to_lowercase().as_str())
        && rand::thread_rng().gen_ratio(percent_chance_of_comeback, 100)
    {
        let mut random_comeback = comeback_options.choose(&mut rand::thread_rng());

        if let Some(comeback) = random_comeback.take() {
            let _ = say(bot_state.clone(), comeback.value.as_str()).await;
            return true;
        }
    }

    false
}

pub mod api {
    use tauri::Manager;

    use crate::bot::{BotData, Comebacks};
    use crate::file::{write_file, WriteFileError};

    #[tauri::command]
    pub fn save_comebacks(
        app_handle: tauri::AppHandle,
        comebacks: Comebacks,
    ) -> Result<(), String> {
        let state = app_handle.state::<BotData>();
        *state
            .comebacks
            .lock()
            .expect("Failed to get lock for bot info") = comebacks.clone();

        let write_result = write_file::<Comebacks>(&app_handle, "comebacks.json", comebacks);

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
        }

        Ok(())
    }
}
