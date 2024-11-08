use std::sync::Mutex;

use super::{Comebacks, Insults, Users};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BotData {
    pub comebacks: Mutex<Comebacks>,
    pub insults: Mutex<Insults>,
    pub users: Mutex<Users>,
}

impl BotData {
    pub fn new(comebacks: Comebacks, insults: Insults, users: Users) -> Self {
        Self {
            comebacks: Mutex::new(comebacks),
            insults: Mutex::new(insults),
            users: Mutex::new(users),
        }
    }
}

impl Default for BotData {
    fn default() -> Self {
        Self {
            comebacks: Mutex::new(Comebacks::default()),
            insults: Mutex::new(Insults::default()),
            users: Mutex::new(Users::default()),
        }
    }
}
