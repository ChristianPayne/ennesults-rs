use std::collections::HashSet;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::{thread, time::Duration};

use serde_inline_default::serde_inline_default;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use crate::bot::{get_random_user, BotData};
use crate::date::{
    date_time_is_greater_than_reference, get_date_time_minutes_ago, get_local_now, parse_date_time,
};

use super::{say, Bot, User};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Insults(pub Vec<Insult>);

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Insult {
    pub id: String,
    pub value: String,
    #[serde(default = "HashSet::new")]
    pub tags: HashSet<InsultTag>,
}

#[derive(Debug, Default)]
pub enum InsultThread {
    Running {
        handle: tokio::task::JoinHandle<()>,
        sender: std::sync::mpsc::Sender<()>,
    },
    #[default]
    Stopped,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, TS, PartialEq, Eq, Hash)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum InsultTag {
    Insult,
    Consent,
    Unconsent,
    Raid,
    Lurk,
}

pub enum InsultThreadShutdownError {
    ThreadNotRunning,
}

impl InsultThread {
    pub fn new(app_handle: tauri::AppHandle, start_thread: bool) -> Self {
        if start_thread {
            let (tx, rx) = mpsc::channel();
            let thread_handle = tokio::spawn(insult_thread_loop(app_handle, rx));

            Self::Running {
                handle: thread_handle,
                sender: tx,
            }
        } else {
            Self::Stopped
        }
    }

    pub fn shutdown(&mut self) -> Result<(), InsultThreadShutdownError> {
        match self {
            InsultThread::Stopped => Err(InsultThreadShutdownError::ThreadNotRunning),
            InsultThread::Running { handle, sender } => {
                sender.send(());
                *self = InsultThread::Stopped;
                Ok(())
            }
        }
    }
}

pub async fn insult_thread_loop(app_handle: AppHandle, rx: Receiver<()>) {
    println!("Starting new insult thread!");
    'thread_loop: loop {
        let state = app_handle.state::<Bot>();

        let (enable_insults, time_between_insults) = {
            let bot_info = state
                .bot_info
                .lock()
                .expect("Failed to get lock for bot_info");

            (bot_info.enable_insults, bot_info.time_between_insults)
        };

        let sleep_time = Duration::from_secs(time_between_insults as u64);
        thread::sleep(sleep_time);

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Shutting down insult thread.");
                break 'thread_loop;
            }
            Err(TryRecvError::Empty) => {}
        }

        let insults = {
            let insults = state
                .bot_data
                .insults
                .lock()
                .expect("Failed to get lock for insults.");

            insults.0.clone()
        };

        // Pick a random insult.
        let random_insult = choose_random_insult(app_handle.clone());

        match random_insult {
            Some(insult) => {
                let formatted_insult = format_insult(app_handle.clone(), &insult);

                match formatted_insult {
                    None => continue 'thread_loop,
                    Some(insult) => {
                        // Say it in chat.
                        let _ = say(state.clone(), insult.as_str()).await;
                    }
                }
            }
            None => {
                println!("Could not get a random insult.")
            }
        }
        println!("Looping insult thread.");
    }
}

pub fn choose_random_insult(app_handle: tauri::AppHandle) -> Option<Insult> {
    let state = app_handle.state::<Bot>();
    let insults = state
        .bot_data
        .insults
        .lock()
        .expect("Failed to get lock for insults");

    // Pick a random insult.
    let rand_insult = insults.0.choose(&mut rand::thread_rng());
    rand_insult.cloned()
}

pub fn format_insult(app_handle: tauri::AppHandle, insult: &Insult) -> Option<String> {
    let state = app_handle.state::<Bot>();
    let mut formatted_message = insult.value.clone();

    if formatted_message.contains("{{streamer}}") {
        let channel_name = {
            let state = app_handle.state::<Bot>();
            let bot_info = state.bot_info.lock().expect("Failed to get bot_info");
            bot_info.channel_name.clone()
        };

        formatted_message = formatted_message.replace("{{streamer}}", channel_name.as_str())
    }

    if formatted_message.contains("{{user}}") {
        let mut users = {
            let users_state = state
                .bot_data
                .users
                .lock()
                .expect("Failed to get lock for users.");
            users_state.clone()
        };

        while formatted_message.contains("{{user}}") {
            let random_user = {
                get_random_user(
                    app_handle.clone(),
                    !insult.value.contains("{{streamer}}"),
                    &users,
                )
                .cloned()
            };

            if let Some(user) = &random_user {
                let username = user.username.clone();
                users.0.remove(&user.username);
            }

            match random_user {
                Some(user) => {
                    formatted_message =
                        formatted_message.replacen("{{user}}", user.username.as_str(), 1);
                }
                None => {
                    println!("No users available after filters to insult.");
                    return None;
                }
            }
        }
    }

    Some(formatted_message)
}

pub mod api {
    use tauri::{Emitter, Manager};

    use crate::bot::{Bot, BotData};
    use crate::file::{write_file, WriteFileError};

    use super::{Insult, InsultTag, Insults};

    #[tauri::command]
    pub fn get_insults(app_handle: tauri::AppHandle) -> Vec<Insult> {
        let state = app_handle.state::<Bot>();
        let insults = {
            state
                .bot_data
                .insults
                .lock()
                .expect("Failed to get lock for insults.")
                .0
                .clone()
        };

        insults
    }

    #[tauri::command]
    pub fn get_insults_count(app_handle: tauri::AppHandle) -> u32 {
        let state = app_handle.state::<Bot>();
        let insults = state
            .bot_data
            .insults
            .lock()
            .expect("Failed to get lock for insults.");

        insults.0.len() as u32
    }

    #[tauri::command]
    pub fn update_insult(app_handle: tauri::AppHandle, insult: Insult) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let mut insults = state
            .bot_data
            .insults
            .lock()
            .expect("Failed to get lock for insults.")
            .clone();

        match insults.0.iter_mut().find(|i| i.id == insult.id) {
            Some(insult_in_db) => {
                *insult_in_db = insult;
            }
            None => {
                return Err("Failed to find insult in database.".to_string());
            }
        }
        save_insults(app_handle, insults)?;

        Ok(())
    }

    #[tauri::command]
    pub fn save_insults(app_handle: tauri::AppHandle, insults: Insults) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        *state
            .bot_data
            .insults
            .lock()
            .expect("Failed to get lock for bot info") = insults.clone();

        let write_result = write_file::<Insults>(&app_handle, "insults.json", insults.clone());

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
            let _ = app_handle.emit("insults_update", insults);
        }

        Ok(())
    }

    #[tauri::command]
    pub fn delete_insult(app_handle: tauri::AppHandle, insult_id: String) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        let insults = {
            let mut insults = state
                .bot_data
                .insults
                .lock()
                .expect("Failed to get lock for insults");

            match insults.0.iter().position(|insult| insult.id == insult_id) {
                None => return Err("Could not find index of insult.".to_string()),
                Some(index) => insults.0.remove(index),
            };

            insults.clone()
        };

        save_insults(app_handle.clone(), insults);

        Ok(())
    }
}
