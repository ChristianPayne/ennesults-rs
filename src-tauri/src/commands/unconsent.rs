use tauri::{AppHandle, Emitter, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::{
    bot::{choose_random_insult, format_insult, Bot, BotData, FormattingOptions, InsultTag, User},
    file::write_file,
};

use super::{meets_minimum_user_level, parse_msg_for_user_level, Command, UserLevel};

#[derive(Debug)]
pub struct UnconsentCommand;

impl Command for UnconsentCommand {
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
        let mut users = state
            .bot_data
            .users
            .lock()
            .expect("Failed to get lock for bot data.");

        let consent_target = match args.len() {
            0 => Some(msg.sender.name.clone()),
            1 => {
                if !meets_minimum_user_level(parse_msg_for_user_level(msg), UserLevel::Moderator) {
                    return Some(format!(
                        "{}, you cannot unconsent for someone else.",
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
                return Some("Failed to parse command!".to_string());
            }
            Some(target) => match users.0.get_mut(&target) {
                None => Some("User not found in the database.".to_string()),
                Some(user) => {
                    if !user.consented {
                        Some(format!("{} is not consented!", &user.username))
                    } else {
                        user.consented = false;

                        // Pick a random insult.
                        let insult = match choose_random_insult(
                            app_handle.clone(),
                            Some(vec![InsultTag::Unconsent]),
                        ) {
                            Some(insult) => format_insult(
                                app_handle.clone(),
                                &insult,
                                FormattingOptions::Unconsent { user: user.clone() },
                            ),
                            None => None,
                        };

                        match insult {
                            None => Some(format!("{}, unconsented!", &user.username)),
                            Some(insult) => Some(insult),
                        }
                    }
                }
            },
        };

        if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
            println!("Failed to write users.json file to disk! {:?}", error);
            let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
        } else {
            let _ = app_handle.emit(
                "users_update",
                users.0.clone().into_values().collect::<Vec<User>>(),
            );
        }

        command_reply
    }
}
