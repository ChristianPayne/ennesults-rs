use rand::seq::SliceRandom;
use std::{
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
    thread,
    time::Duration,
};
use tauri::{AppHandle, Manager};
use tokio::task::JoinHandle;
use ts_rs::TS;

use super::{say, Bot};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Announcements(pub Vec<Announcement>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Announcement {
    pub id: String,
    pub value: String,
}

#[derive(Debug, Default)]
pub enum AnnouncementThread {
    Running {
        handle: JoinHandle<()>,
        sender: Sender<()>,
    },
    #[default]
    Stopped,
}

pub enum AnnouncementThreadShutdownError {
    ThreadNotRunning,
}

impl AnnouncementThread {
    pub fn new(app_handle: tauri::AppHandle, start_thread: bool) -> Self {
        if start_thread {
            let (tx, rx) = mpsc::channel();
            let thread_handle = tokio::spawn(announcement_thread_loop(app_handle, rx));

            Self::Running {
                handle: thread_handle,
                sender: tx,
            }
        } else {
            Self::Stopped
        }
    }
}

pub async fn announcement_thread_loop(app_handle: AppHandle, rx: Receiver<()>) {
    println!("Starting new announcement thread!");

    let mut next_announcement_index: usize = 0;

    loop {
        let state = app_handle.state::<Bot>();
        let (randomize_announcements, time_between_announcements) = {
            let settings = state
                .settings
                .lock()
                .expect("Failed to get lock for settings");

            (
                settings.randomize_announcements,
                settings.time_between_announcements,
            )
        };

        let sleep_time = Duration::from_secs(time_between_announcements as u64);
        thread::sleep(sleep_time);

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Shutting down announcements thread.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        let state = app_handle.state::<Bot>();

        let announcements = {
            let announcements = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for insults.");

            announcements.0.clone()
        };

        // Pick a random announcement.
        let announcement = {
            if randomize_announcements {
                announcements.choose(&mut rand::thread_rng())
            } else {
                // Next announcement
                if announcements.is_empty() {
                    None
                } else {
                    let x = &announcements[next_announcement_index];

                    next_announcement_index += 1;

                    if next_announcement_index == announcements.len() {
                        next_announcement_index = 0
                    }

                    Some(x)
                }
            }
        };

        match announcement {
            Some(announcement) => {
                // Say it in chat.
                let _ = say(state.clone(), announcement.value.as_str()).await;
            }
            None => println!("Could not get an announcement to say."),
        }

        println!("Looping announcement thread.");
    }
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::Bot;
    use crate::file::{write_file, WriteFileError};

    use super::{Announcement, Announcements};

    #[tauri::command]
    pub fn get_announcements(app_handle: tauri::AppHandle) -> Vec<Announcement> {
        let state = app_handle.state::<Bot>();
        let announcements = {
            state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements.")
                .0
                .clone()
        };

        announcements
    }

    #[tauri::command]
    pub fn update_announcement(
        app_handle: tauri::AppHandle,
        announcement: Announcement,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut announcements = state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for announcements.")
            .clone();

        match announcements.0.iter_mut().find(|i| i.id == announcement.id) {
            Some(announcement_in_db) => {
                announcement_in_db.value = announcement.value;
            }
            None => return Err("Failed to find announcement in database".to_string()),
        }

        save_announcements(app_handle, announcements)?;

        Ok(())
    }

    #[tauri::command]
    pub fn save_announcements(
        app_handle: tauri::AppHandle,
        announcements: Announcements,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        *state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for settings") = announcements.clone();

        let write_result =
            write_file::<Announcements>(&app_handle, "announcements.json", announcements.clone());

        if let Some(err) = write_result.err() {
            match err {
                WriteFileError::FailedConvertJSON => {
                    return Err("Failed to convert to json.".to_string())
                }
                WriteFileError::FailedCreateFile => {
                    return Err("Failed to create file.".to_string())
                }
                WriteFileError::FailedWriteFile => {
                    return Err("Failed to write contents in file.".to_string())
                }
            }
        } else {
            let _ = app_handle.emit("announcements_update", announcements);
        }

        Ok(())
    }

    #[tauri::command]
    pub fn delete_announcement(
        app_handle: tauri::AppHandle,
        announcement_id: String,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let announcements = {
            let mut announcements = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements");
            match announcements
                .0
                .iter()
                .position(|announcement| announcement.id == announcement_id)
            {
                None => return Err("Could not find index of insult.".to_string()),
                Some(index) => announcements.0.remove(index),
            };

            announcements.clone()
        };

        let _ = save_announcements(app_handle.clone(), announcements);

        Ok(())
    }
}
