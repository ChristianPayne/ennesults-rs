use tauri::AppHandle;
use twitch_irc::message::PrivmsgMessage;

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct TestCommand;

impl Command for TestCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Creator
    }
    fn run(
        &self,
        _args: Vec<String>,
        _msg: &PrivmsgMessage,
        _app_handle: AppHandle,
    ) -> Option<String> {
        Some("What are we testing?".to_string())
    }
}
