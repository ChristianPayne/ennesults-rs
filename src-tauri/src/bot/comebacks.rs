use tauri::{AppHandle, Manager};
use ts_rs::TS;
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{say, Bot, BotInfo};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Comebacks(pub Vec<Comeback>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Comeback {
    pub id: u16,
    pub value: String,
}

pub async fn process_comebacks(app_handle: AppHandle, msg: &PrivmsgMessage) {
    let bot_state = app_handle.state::<Bot>();

    let (bot_name, percent_chance_of_comeback) = {
        let bot_info = bot_state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        // Check to make sure comebacks are enabled in the settings.
        if !bot_info.enable_comebacks {
            return;
        }
        // Get bot name
        (
            bot_info.bot_name.clone(),
            bot_info.percent_chance_of_comeback,
        )
    };

    // Check if bot name is in msg.
    if msg
        .message_text
        .to_lowercase()
        .contains(bot_name.to_lowercase().as_str())
    {
        // Random chance to say a comeback.
        // Use the settings value for the max chance value.
        percent_chance_of_comeback;

        // Random comeback.
        let _ = say(bot_state.clone(), "Yes? Can I help you?").await;
    }
}
