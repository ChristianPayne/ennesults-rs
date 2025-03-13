use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use crate::bot::{
    api::{connect_to_channel, connect_to_twitch},
    run_insult, Bot,
};

use super::{Command, UserLevel};

#[derive(Debug)]
pub struct DiagnosticCommand;

impl Command for DiagnosticCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Creator
    }
    fn run(
        &self,
        args: Vec<String>,
        _msg: &PrivmsgMessage,
        app_handle: AppHandle,
    ) -> Option<String> {
        let state = app_handle.state::<Bot>();

        match args.len() {
            1 => match args[0].as_str() {
                "message_thread" => {
                    let client = state.client.lock().unwrap();
                    let is_message_thread_running = client.is_message_thread_running();

                    if is_message_thread_running {
                        let message = format!(
                            "{} Message thread running",
                            status_emoji(is_message_thread_running)
                        );
                        println!("{}", &message);
                        Some(message)
                    } else {
                        let message = format!(
                            "{} Message thread not running",
                            status_emoji(is_message_thread_running)
                        );
                        println!("{}", &message);
                        Some(message)
                    }
                }
                "insults" => {
                    let insults = state
                        .bot_data
                        .insults
                        .lock()
                        .expect("Failed to get insults lock.");
                    let message = format!("{} Insults", status_emoji(!insults.0.is_empty()));
                    println!("{}", &message);
                    Some(message)
                }
                "announcements" => {
                    let announcements = state
                        .bot_data
                        .announcements
                        .lock()
                        .expect("Failed to get announcements lock.");
                    let message = format!(
                        "{} Announcements",
                        status_emoji(!announcements.announcements.is_empty())
                    );
                    println!("{}", &message);
                    Some(message)
                }
                _ => None,
            },
            2 => match args[0].as_str() {
                "run" => match args[1].as_str() {
                    "reconnect" => {
                        tokio::spawn(async move {
                            let _ = connect_to_twitch(app_handle.clone()).await;
                            let _ = connect_to_channel(app_handle.clone()).await;
                        });
                        Some("🔄 Reconnecting...".to_string())
                    }
                    "insult" => run_insult(app_handle.clone()),
                    _ => Some("🤔 Sub command not found".to_string()),
                },
                _ => Some("🔍 diagnostics: run argument not found.".to_string()),
            },
            _ => Some(
                "🔍 diagnostics: message_thread, insults, announcements, run <reconnect, insult>"
                    .to_string(),
            ),
        }
    }
}

fn status_emoji(status: bool) -> String {
    if status {
        "✅".to_string()
    } else {
        "❌".to_string()
    }
}
