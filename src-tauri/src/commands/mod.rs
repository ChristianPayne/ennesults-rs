pub mod connect_to_channel;
pub mod hello_world;
pub mod leave_channel;
pub mod say;
pub mod status;
pub mod bot_api;

// Command components
#[derive(Clone)]
pub struct CommandName(String);
pub enum AccessLevel {
    Streamer,
    Moderator,
    User,
}

pub enum CommandTypes {
    Say,
    Action,
}

trait Command {
    fn get_name(self) -> CommandName;
    fn get_aliases(self) -> Vec<&'static str>;
}
