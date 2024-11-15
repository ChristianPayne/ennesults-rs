use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::Bot;

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

        let client = bot.client.lock().expect("Failed to get lock for client");

        match &client.insult_thread_sender {
            None => (),
            Some(tx) => {
                let _ = tx.send(());
                ()
            }
        }

        Some("Shutting down insult thread! ⚠️".into())
    }
}
