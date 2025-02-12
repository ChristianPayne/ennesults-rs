use rand::Rng;
use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use super::{say, Bot};

pub async fn process_corrections(app_handle: AppHandle, msg: &PrivmsgMessage) -> bool {
    if !msg.message_text.to_lowercase().contains("en") {
        return false;
    }

    let state = app_handle.state::<Bot>();

    // Get values from state and lock the value back up.
    let (correction_exceptions, percent_chance_of_correction) = {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings.");

        // Check to make sure comebacks are enabled in the settings.
        if !settings.enable_corrections {
            return false;
        }

        (
            settings.correction_exceptions.clone(),
            settings.percent_chance_of_correction,
        )
    };

    let contains_exception = correction_exceptions
        .iter()
        .any(|exception| msg.message_text.to_lowercase().contains(exception));

    // Get random percent chance.
    if !contains_exception && rand::thread_rng().gen_ratio(percent_chance_of_correction, 100) {
        let corrected_message = format!(
            "Correction: {}",
            msg.message_text.to_lowercase().replace("en", "ENNE")
        );

        let _ = say(state, corrected_message.as_str()).await;

        return true;
    }

    false
}
