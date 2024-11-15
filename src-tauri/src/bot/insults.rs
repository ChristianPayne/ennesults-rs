use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread, time::Duration};

use tauri::{AppHandle, Manager};
use ts_rs::TS;

use super::{say, Bot};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Insults(Vec<Insult>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Insult {
    id: u16,
    value: String,
}

pub async fn insult_thread_loop(app_handle: AppHandle, rx: Receiver<()>) {
    println!("Started Loop!");
    loop {
        println!("Running loop!");
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        let state = app_handle.state::<Bot>();

        let enable_insults = {
            let bot_info = state
                .bot_info
                .lock()
                .expect("Failed to get lock for bot_info");

            bot_info.enable_insults
        };

        if enable_insults {
            say(state, "Sending an insult.").await;
        }

        let sleep_time = Duration::from_secs(5);
        thread::sleep(sleep_time);
    }
}
