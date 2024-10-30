mod test;
use test::*;

use ts_rs::TS;

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
    fn run(&self) -> Option<String>;
}

pub fn command_from_str(input: &str) -> Option<Box<dyn Command>> {
    match input {
        "test" | "t" => Some(Box::new(TestCommand)),
        _ => None,
    }
}
