use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{api::get_users, Bot};

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
        app_handle: AppHandle,
    ) -> Option<String> {
        let state = app_handle.state::<Bot>();
        let users = state.bot_data.get_users();

        dbg!(users);
        None
    }
}
