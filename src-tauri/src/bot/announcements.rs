use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};
use ts_rs::TS;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use super::{say, Bot, User};

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
        handle: tokio::task::JoinHandle<()>,
        sender: std::sync::mpsc::Sender<()>,
    },
    #[default]
    Stopped,
}

pub enum AnnouncementThreadShutdownError {
    ThreadNotRunning,
}

impl AnnouncementThread {
    pub fn from(
        announcement_thread_handle: Option<tokio::task::JoinHandle<()>>,
        announcement_thread_sender: Option<std::sync::mpsc::Sender<()>>,
    ) -> AnnouncementThread {
        match (announcement_thread_handle, announcement_thread_sender) {
            (Some(handle), Some(sender)) => AnnouncementThread::Running { handle, sender },
            (_, _) => AnnouncementThread::Stopped,
        }
    }
}

pub async fn announcement_thread_loop(app_handle: AppHandle, rx: Receiver<()>) {
    println!("Starting new announcement thread!");

    let mut next_announcement_index: usize = 0;

    loop {
        let state = app_handle.state::<Bot>();
        let (enable_announcements, randomize_announcements, time_between_announcements) = {
            let bot_info = state
                .bot_info
                .lock()
                .expect("Failed to get lock for bot_info");

            (
                bot_info.enable_announcements,
                bot_info.randomize_announcements,
                bot_info.time_between_announcements,
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

        if enable_announcements {
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
        }

        println!("Looping announcement thread.");
    }
}

pub mod api {}
