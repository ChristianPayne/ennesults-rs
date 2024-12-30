use std::ops::{Deref, DerefMut};

use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{Bot, Client, InsultThread};

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct TestCommand;

impl Command for TestCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Creator
    }
    fn run(
        &self,
        args: Vec<String>,
        msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        Some("What are we testing?".to_string())
    }
}
