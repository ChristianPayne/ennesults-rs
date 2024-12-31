mod announcements;
mod bot;
mod bot_data;
mod bot_info;
mod client;
mod comebacks;
mod corrections;
mod insults;
mod users;
mod whispers;

pub use announcements::*;
pub use bot::*;
pub use bot_data::*;
pub use bot_info::*;
pub use client::*;
pub use comebacks::*;
pub use corrections::*;
pub use insults::*;
pub use users::*;
pub use whispers::*;

pub mod api {
    use super::announcements;
    use super::bot;
    use super::bot_info;
    use super::client;
    use super::comebacks;
    use super::insults;
    use super::users;
    use super::whispers;

    pub use announcements::api::*;
    pub use bot::api::*;
    pub use bot_info::api::*;
    pub use client::api::*;
    pub use comebacks::api::*;
    pub use insults::api::*;
    pub use users::api::*;
    pub use whispers::api::*;
}
