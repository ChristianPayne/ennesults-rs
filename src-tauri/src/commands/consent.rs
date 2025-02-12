use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{choose_random_insult, format_insult, Bot, InsultTag};

use super::{meets_minimum_user_level, parse_msg_for_user_level, Command, UserLevel};

#[derive(Debug)]
pub struct ConsentCommand;

impl Command for ConsentCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Viewer
    }

    fn run(
        &self,
        args: Vec<String>,
        msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        let state = app_handle.state::<Bot>();
        let mut users = state.bot_data.get_users();

        let consent_target = match args.len() {
            0 => Some(msg.sender.name.clone()),
            1 => {
                if !meets_minimum_user_level(parse_msg_for_user_level(msg), UserLevel::Moderator) {
                    return Some(format!(
                        "{}, you cannot consent for someone else.",
                        msg.sender.name,
                    ));
                }
                let mut target = args[0].clone();

                if target.starts_with('@') {
                    target = target.split_off(1)
                }

                Some(target)
            }
            _ => None,
        };

        let command_reply = match consent_target {
            None => {
                return Some("Failed to parse consent!".to_string());
            }
            Some(target) => match users.0.get_mut(&target) {
                None => Some("User not found in the database.".to_string()),
                Some(user) => {
                    if user.consented {
                        Some(format!("{} has already consented!", &user.username))
                    } else {
                        user.consented = true;

                        // Pick a random insult.
                        let insult = match choose_random_insult(
                            app_handle.clone(),
                            Some(vec![InsultTag::Consent]),
                        ) {
                            Some(insult) => {
                                format_insult(app_handle.clone(), &insult, Some(user.clone()), None)
                            }
                            None => None,
                        };

                        match insult {
                            None => Some(format!("Consented, {}!", &user.username)),
                            Some(insult) => Some(insult),
                        }
                    }
                }
            },
        };

        let _ = state.bot_data.save_users(app_handle.clone(), &users);

        command_reply
    }
}
