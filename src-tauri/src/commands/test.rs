use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::Bot;

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct TestCommand;

impl Command for TestCommand {
    fn run(
        &self,
        _args: Vec<String>,
        _msg: &PrivmsgMessage,
        _app_handle: AppHandle,
    ) -> Option<String> {
        None
    }
}
