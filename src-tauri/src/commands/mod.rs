mod connect_to_channel;
mod leave_channel;
mod say;
mod get_channel_status;
mod bot_api;
mod print_bot_data;
// mod hello_world;

pub use connect_to_channel::*;
pub use leave_channel::*;
pub use say::*;
pub use get_channel_status::*;
pub use bot_api::*;
pub use print_bot_data::*;
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
