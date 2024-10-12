mod bot_api;
mod connect_to_channel;
mod get_channel_status;
mod leave_channel;
mod print_bot_data;
mod say;
// mod hello_world;

pub use bot_api::*;
pub use connect_to_channel::*;
pub use get_channel_status::*;
pub use leave_channel::*;
pub use print_bot_data::*;
pub use say::*;
// pub use hello_world::*;

// // Command components
// #[derive(Clone)]
// pub struct CommandName(String);
// pub enum AccessLevel {
//     Streamer,
//     Moderator,
//     User,
// }

// pub enum CommandTypes {
//     Say,
//     Action,
// }

// trait Command {
//     fn get_name(self) -> CommandName;
//     fn get_aliases(self) -> Vec<&'static str>;
// }
