mod announcements;
mod auth;
mod bot;
mod bot_data;
mod client;
mod comebacks;
mod corrections;
mod insults;
mod message_thread;
mod settings;
mod users;
mod whispers;

pub use announcements::*;
pub use auth::*;
pub use bot::*;
pub use bot_data::*;
pub use client::*;
pub use comebacks::*;
pub use corrections::*;
pub use insults::*;
pub use message_thread::*;
pub use settings::*;
pub use users::*;
pub use whispers::*;

pub mod api {
    use super::announcements;
    use super::auth;
    use super::bot;
    use super::client;
    use super::comebacks;
    use super::insults;
    use super::settings;
    use super::users;
    use super::whispers;

    pub use announcements::api::*;
    pub use auth::api::*;
    pub use bot::api::*;
    pub use client::api::*;
    pub use comebacks::api::*;
    pub use insults::api::*;
    pub use settings::api::*;
    pub use users::api::*;
    pub use whispers::api::*;
}
