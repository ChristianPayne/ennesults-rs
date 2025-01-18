use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[serde(default = "Default::default")]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct BotInfo {
    pub channel_name: String,
    // pub bot_name: String,
    // pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub enable_whispers: bool,
    pub users_allowed_to_whisper: Vec<String>,

    pub enable_announcements: bool,
    pub randomize_announcements: bool,
    pub time_between_announcements: u32,

    pub enable_insults: bool,
    pub time_between_insults: u32,
    pub lurk_time: u32,

    pub enable_comebacks: bool,
    pub percent_chance_of_comeback: u32,
    pub comeback_exceptions: Vec<String>,

    pub enable_corrections: bool,
    pub percent_chance_of_correction: u32,
    pub correction_exceptions: Vec<String>,
}

impl Default for BotInfo {
    fn default() -> Self {
        Self {
            channel_name: "".into(),
            // bot_name: "".into(),
            // oauth_token: "".into(),
            auto_connect_on_startup: false,
            enable_whispers: false,
            users_allowed_to_whisper: vec![],
            enable_announcements: false,
            randomize_announcements: false,
            time_between_announcements: 300,
            enable_insults: false,
            time_between_insults: 300,
            lurk_time: 5,
            enable_comebacks: false,
            percent_chance_of_comeback: 20,
            comeback_exceptions: vec![],
            enable_corrections: false,
            percent_chance_of_correction: 20,
            correction_exceptions: vec![],
        }
    }
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::api::{connect_to_channel, connect_to_twitch};
    use crate::bot::{Bot, BotInfo};
    use crate::file::{write_file, WriteFileError};

    #[tauri::command]
    pub fn get_channel_name(state: tauri::State<'_, Bot>) -> Result<String, String> {
        Ok(state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info")
            .channel_name
            .clone())
    }

    #[tauri::command]
    pub fn get_bot_info(state: tauri::State<'_, Bot>) -> BotInfo {
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info")
            .clone();
        bot_info
    }

    #[tauri::command]
    pub async fn save_bot_info(
        app_handle: tauri::AppHandle,
        bot_info: BotInfo,
    ) -> Result<BotInfo, String> {
        let state = app_handle.state::<Bot>();
        let mut bot_info = bot_info;
        bot_info.channel_name = bot_info.channel_name.to_lowercase();
        {
            *state
                .bot_info
                .lock()
                .expect("Failed to get lock for bot info") = bot_info.clone();
        }

        connect_to_twitch(app_handle.clone());

        if bot_info.auto_connect_on_startup {
            let _ = connect_to_channel(app_handle.clone()).await;
        }

        let write_result = write_file::<BotInfo>(&app_handle, "bot_info.json", bot_info.clone());

        if let Some(err) = write_result.err() {
            return match err {
                WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
                WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
                WriteFileError::FailedWriteFile => {
                    Err("Failed to write contents in file.".to_string())
                }
            };
        }

        let _ = app_handle.emit("bot_info_save", bot_info.clone());

        Ok(bot_info)
    }
}
