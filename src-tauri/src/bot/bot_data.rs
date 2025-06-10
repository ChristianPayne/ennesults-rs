use crate::{
    bot::{
        announcements::Announcements,
        comebacks::Comebacks,
        insults::Insults,
        users::{User, Users},
    },
    helpers::file::{write_file, WriteFileError},
};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

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

    /// Clones the users struct and returns it.
    pub fn get_users(&self) -> Users {
        let users_guard = self.users.lock().expect("Failed to get lock for users.");

        users_guard.clone()
    }

    pub fn save_users(&self, app_handle: AppHandle, users: &Users) -> Result<(), WriteFileError> {
        let mut users_guard = self.users.lock().expect("Failed to get lock for users.");

        if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
            println!("Failed to write users.json file to disk! {:?}", error);
            let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
            Err(error)
        } else {
            let _ = app_handle.emit(
                "users_update",
                users.0.clone().into_values().collect::<Vec<User>>(),
            );
            *users_guard = users.clone();
            Ok(())
        }
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
