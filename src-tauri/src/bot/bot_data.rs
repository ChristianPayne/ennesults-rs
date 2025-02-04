use std::sync::Mutex;

use super::{Announcements, Comebacks, Insults, Users};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BotData {
    pub comebacks: Mutex<Comebacks>,
    pub insults: Mutex<Insults>,
    pub users: Mutex<Users>,
    pub announcements: Mutex<Announcements>,
}

impl BotData {
    pub fn new(
        comebacks: Comebacks,
        insults: Insults,
        users: Users,
        announcements: Announcements,
    ) -> Self {
        Self {
            comebacks: Mutex::new(comebacks),
            insults: Mutex::new(insults),
            users: Mutex::new(users),
            announcements: Mutex::new(announcements),
        }
    }

    pub fn get_users(&self) -> Users {
        let users_guard = self.users.lock().expect("Failed to get lock for users.");

        users_guard.clone()
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            comebacks: Mutex::new(Comebacks::default()),
            insults: Mutex::new(Insults::default()),
            users: Mutex::new(Users::default()),
            announcements: Mutex::new(Announcements::default()),
        }
    }
}
