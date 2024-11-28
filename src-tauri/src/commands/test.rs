use std::ops::{Deref, DerefMut};

use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{Bot, Client, InsultThread};

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct TestCommand;

impl Command for TestCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Moderator
    }
    fn run(
        &self,
        args: Vec<String>,
        msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        let bot = app_handle.state::<Bot>();

        let mut client = bot.client.lock().expect("Failed to get lock for client");

        match client.deref_mut() {
            Client::Disconnected => None,
            Client::Connected {
                client,
                client_join_handle,
                insult_thread,
            } => match insult_thread.shutdown() {
                Ok(_) => Some("Insult thread shut down.".into()),
                Err(err) => match err {
                    crate::bot::InsultThreadShutdownError::ThreadNotRunning => {
                        Some("Insult thread not running".into())
                    }
                },
            },
        }
    }
}
