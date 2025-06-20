use tauri::AppHandle;
use ts_rs::TS;
use twitch_irc::message::PrivmsgMessage;

mod consent;
mod diagnostic;
mod lurk;
mod test;
mod unconsent;
mod version;

use consent::ConsentCommand;
use diagnostic::DiagnosticCommand;
use lurk::LurkCommand;
use test::TestCommand;
use unconsent::UnconsentCommand;
use version::VersionCommand;

#[derive(serde::Serialize, Clone, Copy, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum UserLevel {
    Viewer,
    Subscriber,
    Vip,
    Moderator,
    Broadcaster,
    Creator,
    Bot,
}

impl UserLevel {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub trait Command: Send {
    /// The user level required to get the command to run. All levels under this will not be allowed.
    fn get_required_user_level(&self) -> UserLevel {
        UserLevel::Viewer
    }
    /// This function will run when the command is matched. The optional returned string is the reply that Ennesults will say in chat.
    fn run(
        &self,
        _args: Vec<String>,
        _msg: &PrivmsgMessage,
        _app_handle: AppHandle,
    ) -> Option<String> {
        Some("⚠️ Command still being worked on! ⚠️".to_string())
    }
}

pub fn command_from_str(command_string: &str) -> Option<Box<dyn Command>> {
    match command_string {
        "test" | "t" => Some(Box::new(TestCommand)),
        "consent" | "c" | "consennet" => Some(Box::new(ConsentCommand)),
        "unconsent" | "uc" | "unconsennet" => Some(Box::new(UnconsentCommand)),
        "version" | "v" => Some(Box::new(VersionCommand)),
        "diagnostic" | "d" => Some(Box::new(DiagnosticCommand)),
        "lurk" | "l" => Some(Box::new(LurkCommand)),
        _ => None,
    }
}

pub enum ParseCommandError {
    NotACommand,
    CommandNotFound,
    CommandArgsError,
}

pub fn parse_for_command(
    msg: &PrivmsgMessage,
) -> Result<(Box<dyn Command>, Vec<String>), ParseCommandError> {
    if !msg.message_text.starts_with('!') {
        return Err(ParseCommandError::NotACommand);
    };

    let mut raw_message = msg.message_text.clone();
    let command = raw_message.split_off(1);
    let msg_split: Vec<String> = command
        .split_whitespace()
        // Max allowable arguments are two; command arg1 arg2.
        .take(3)
        .map(|x| x.to_string())
        .collect();

    let [command_name, args @ ..] = &msg_split[..] else {
        return Err(ParseCommandError::CommandArgsError);
    };

    println!("Raw command, split: {}, {:?}", command_name, args);

    let Some(command) = command_from_str(command_name) else {
        return Err(ParseCommandError::CommandNotFound);
    };

    Ok((command, args.to_vec()))
}

pub fn parse_msg_for_user_level(msg: &PrivmsgMessage) -> UserLevel {
    // Creator rule
    if msg.sender.name.to_lowercase() == "chrisgriffin522" {
        return UserLevel::Creator;
    }

    // Convert from badge to UserLevel
    for badge in &msg.badges {
        let badge_name = badge.name.as_str();

        if badge_name == "broadcaster" {
            return UserLevel::Broadcaster;
        }
        if badge_name == "moderator" {
            return UserLevel::Moderator;
        }
        if badge_name == "vip" {
            return UserLevel::Vip;
        }
        if badge_name == "subscriber" {
            return UserLevel::Subscriber;
        }
    }

    UserLevel::Viewer
}

/// Checks a user level against a requirement. If the user level is *at or above* the requirement, this will succeed.
pub fn has_sufficient_permissions(user_level: UserLevel, required_user_level: UserLevel) -> bool {
    user_level.index() >= required_user_level.index()
}
