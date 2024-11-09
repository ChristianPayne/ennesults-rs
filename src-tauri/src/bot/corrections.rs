use rand::Rng;
use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use super::{say, Bot, BotInfo};

pub async fn process_corrections(app_handle: AppHandle, msg: &PrivmsgMessage) {
    let state = app_handle.state::<Bot>();

    // Get values from state and lock the value back up.
    let (correction_exceptions, percent_chance_of_correction) = {
        let bot_info = state
            .bot_info
            .lock()
            .expect("Failed to get lock for bot info.");

        // Check to make sure comebacks are enabled in the settings.
        if !bot_info.enable_corrections {
            return;
        }

        (
            bot_info.correction_exceptions.clone(),
            bot_info.percent_chance_of_correction,
        )
    };

    let contains_exception = correction_exceptions
        .iter()
        .any(|exception| msg.message_text.contains(exception));

    dbg!(&contains_exception, &correction_exceptions);

    // Get random percent chance.
    if !contains_exception && rand::thread_rng().gen_ratio(percent_chance_of_correction, 100) {
        // Corrected message.

        // for word in msg.message_text.split(' ') {
        //     word.replace("en", "ENNE")
        // }

        let corrected_message = msg.message_text.replace("en", "ENNE");

        say(state, corrected_message.as_str()).await;
    }
}
