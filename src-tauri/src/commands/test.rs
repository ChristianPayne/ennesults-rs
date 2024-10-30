use super::{Command, UserLevel};

#[derive(Debug)]
pub struct TestCommand;

impl Command for TestCommand {
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Moderator
    }
    fn run(&self) -> Option<String> {
        Some(
            "Hey! Thanks for testing out the new bot! ⚠️ Ennesults is still under construction! ⚠️"
                .into(),
        )
    }
}
