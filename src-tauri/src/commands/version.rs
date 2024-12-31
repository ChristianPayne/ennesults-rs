use tauri::{AppHandle, Emitter, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::{
    bot::{choose_random_insult, format_insult, Bot, BotData, FormattingOptions, InsultTag, User},
    file::write_file,
};

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct VersionCommand;

impl Command for VersionCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Creator
    }

    fn run(
        &self,
        args: Vec<String>,
        msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        Some(format!(
            "Ennesults is currently on v{} ennegiSults",
            app_handle.package_info().version.clone()
        ))
    }
}
