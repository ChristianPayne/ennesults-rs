use tauri::AppHandle;
use ts_rs::TS;
use twitch_irc::message::PrivmsgMessage;

mod consent;
mod test;
mod unconsent;

use consent::ConsentCommand;
use test::TestCommand;
use unconsent::UnconsentCommand;

#[derive(serde::Serialize, Clone, Copy, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum UserLevel {
    Viewer,
    Subscriber,
    Vip,
    Moderator,
    Broadcaster,
}

impl UserLevel {
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub trait Command: Send {
    /// The user level required to get the command to run. All levels under this will not be allowed.
    fn get_required_user_level(&self) -> UserLevel;
    /// This function will run when the command is matched. The optional returned string is the reply that Ennesults will say in chat.
    fn run(&self, args: Vec<String>, msg: &PrivmsgMessage, app_handle: AppHandle)
        -> Option<String>;
}

pub fn command_from_str(command_string: &str) -> Option<Box<dyn Command>> {
    match command_string {
        "test" | "t" => Some(Box::new(TestCommand)),
        "consent" | "c" | "consennet" => Some(Box::new(ConsentCommand)),
        "unconsent" | "uc" | "unconsennet" => Some(Box::new(UnconsentCommand)),
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
    // println!("{}", &msg.message_text);
    if !msg.message_text.starts_with('!') {
        return Err(ParseCommandError::NotACommand);
    };

    let mut raw_message = msg.message_text.clone();
    let command = raw_message.split_off(1);
    let msg_split: Vec<String> = command
        .split_whitespace()
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

pub fn meets_minimum_user_level(
    incoming_user_level: UserLevel,
    reference_user_level: UserLevel,
) -> bool {
    incoming_user_level.index() >= reference_user_level.index()
}
