use tauri::AppHandle;
use twitch_irc::message::PrivmsgMessage;

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct VersionCommand;

impl Command for VersionCommand {
    fn run(
        &self,
        _args: Vec<String>,
        _msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        Some(format!(
            "Ennesults is currently on v{} ennegiSults",
            app_handle.package_info().version.clone()
        ))
    }
}
