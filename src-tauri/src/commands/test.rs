use tauri::{AppHandle, Manager};
use twitch_irc::message::PrivmsgMessage;

use twitch_api::helix::HelixClient;
use twitch_api::twitch_oauth2::{AccessToken, UserToken};

use crate::bot::Bot;

use super::{Command, UserLevel};

use crate::bot::Client;

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
        let join_handle: tokio::task::JoinHandle<Option<String>> = tokio::spawn(async move {
            println!("Starting test...");

            let state = app_handle.state::<Bot>();
            let details = {
                let auth = state.auth.lock().expect("Failed to lock auth");
                auth.get_authentication_details()
            };

            if let Some(details) = details {
                let client: HelixClient<reqwest::Client> = HelixClient::default();

                let token = UserToken::from_token(&client, AccessToken::from(details.access_token))
                    .await
                    .unwrap();

                let results = client.get_channel_from_login("ennegineer", &token).await;

                println!("Channel: {:?}", results.unwrap());
            }

            None
        });

        println!("Test complete!");
        None
    }
}
