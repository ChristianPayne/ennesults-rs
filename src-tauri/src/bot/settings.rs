use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[serde(default = "Default::default")]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Settings {
    pub channel_name: String,
    // pub bot_name: String,
    // pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub enable_whispers: bool,
    pub users_allowed_to_whisper: Vec<String>,

    pub enable_announcements: bool,
    pub randomize_announcements: bool,
    pub minimum_time_between_announcements: u32,
    pub maximum_time_between_announcements: u32,

    pub enable_insults: bool,
    pub minimum_time_between_insults: u32,
    pub maximum_time_between_insults: u32,
    pub lurk_time: u32,

    pub enable_comebacks: bool,
    pub percent_chance_of_comeback: u32,
    pub comeback_exceptions: Vec<String>,

    pub enable_corrections: bool,
    pub percent_chance_of_correction: u32,
    pub correction_exceptions: Vec<String>,

    pub message_queue_interval: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            channel_name: "".into(),
            auto_connect_on_startup: false,
            enable_whispers: false,
            users_allowed_to_whisper: vec![],
            enable_announcements: false,
            randomize_announcements: false,
            minimum_time_between_announcements: 300,
            maximum_time_between_announcements: 300,
            enable_insults: false,
            minimum_time_between_insults: 300,
            maximum_time_between_insults: 300,
            lurk_time: 5,
            enable_comebacks: false,
            percent_chance_of_comeback: 20,
            comeback_exceptions: vec![],
            enable_corrections: false,
            percent_chance_of_correction: 20,
            correction_exceptions: vec![],
            message_queue_interval: 6,
        }
    }
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::api::{connect_to_channel, connect_to_twitch};
    use crate::bot::{Bot, Settings};
    use crate::helpers::file::{write_file, WriteFileError};

    #[tauri::command]
    pub fn get_channel_name(state: tauri::State<'_, Bot>) -> Result<String, String> {
        Ok(state
            .settings
            .lock()
            .expect("Failed to get lock for settings")
            .channel_name
            .clone())
    }

    #[tauri::command]
    pub fn get_settings(state: tauri::State<'_, Bot>) -> Settings {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings")
            .clone();
        settings
    }

    #[tauri::command]
    pub async fn save_settings(
        app_handle: tauri::AppHandle,
        settings: Settings,
    ) -> Result<Settings, String> {
        let state = app_handle.state::<Bot>();
        let mut settings = settings;
        settings.channel_name = settings.channel_name.to_lowercase();
        {
            *state
                .settings
                .lock()
                .expect("Failed to get lock for settings") = settings.clone();
        }

        let _ = connect_to_twitch(app_handle.clone()).await;

        if settings.auto_connect_on_startup {
            let _ = connect_to_channel(app_handle.clone()).await;
        }

        let write_result = write_file::<Settings>(&app_handle, "settings.json", settings.clone());

        if let Some(err) = write_result.err() {
            return match err {
                WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
                WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
                WriteFileError::FailedWriteFile => {
                    Err("Failed to write contents in file.".to_string())
                }
            };
        }

        let _ = app_handle.emit("settings_save", settings.clone());

        Ok(settings)
    }
}
