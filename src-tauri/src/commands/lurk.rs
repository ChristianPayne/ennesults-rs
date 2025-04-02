use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{choose_random_insult, format_insult, Bot, InsultTag};

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct LurkCommand;

impl Command for LurkCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Viewer
    }

    fn run(
        &self,
        _args: Vec<String>,
        msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        let state = app_handle.state::<Bot>();
        let mut users = state.bot_data.get_users();

        let target = msg.sender.name.clone();

        let formatted_insult = match users.0.get_mut(&target) {
            Some(user) => {
                user.lurk = true;

                if !user.consented {
                    return None;
                }

                // Pick a random insult.
                let insult = choose_random_insult(app_handle.clone(), Some(vec![InsultTag::Lurk]))?;

                // Format for any templates in the insult.
                format_insult(app_handle.clone(), &insult, Some(user.clone()), None)
            }
            None => None,
        };

        let _ = state.bot_data.save_users(app_handle.clone(), &users);

        formatted_insult
    }
}
