use std::sync::Mutex;

use super::{Users, Comebacks, Insults};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BotData {
    pub comebacks: Mutex<Comebacks>,
    pub insults: Mutex<Insults>,
    pub users: Mutex<Users>,
    pub users_allowed_to_whisper: Mutex<Vec<String>>
}

impl BotData {
    pub fn new(comebacks: Comebacks, insults: Insults, users: Users, users_allowed_to_whisper: Vec<String>) -> Self {
        Self {
            comebacks: Mutex::new(comebacks),
            insults: Mutex::new(insults),
            users: Mutex::new(users),
            users_allowed_to_whisper: Mutex::new(users_allowed_to_whisper)
        }
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            comebacks: Mutex::new(Comebacks::default()),
            insults: Mutex::new(Insults::default()),
            users: Mutex::new(Users::default()),
            users_allowed_to_whisper: Mutex::new(Vec::new())
        }
    }
}