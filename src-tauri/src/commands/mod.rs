use std::str::FromStr;
mod test;
use test::*;

use ts_rs::TS;

#[derive(serde::Serialize, Clone, Debug, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum UserLevel {
    Viewer,
    Vip,
    Moderator,
    Streamer,
}

pub trait Command: Send {
    fn get_name(&self) -> String;
    fn get_aliases(&self) -> Option<Vec<String>>;
    fn get_required_user_level(&self) -> UserLevel;
    fn run(&self) -> Option<String>;
}

// unsafe impl Send for Command {}

pub fn command_from_str(input: &str) -> Option<Box<dyn Command>> {
    match input {
        "test" => Some(Box::new(Test)),
        "second" => Some(Box::new(SecondTest)),
        _ => None,
    }
}
