use crate::helpers::{date::get_local_now, queue::Queue};
use chrono::{DateTime, Local};
use rand::Rng;
use std::thread;
use std::time::Duration;
use tauri::Manager;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;

use super::{run_announcement, run_insult, say, Bot};

#[derive(Debug, Default)]
pub enum MessageThread {
    Running {
        handle: JoinHandle<()>,
        sender: Sender<MessageThreadMessage>,
    },
    #[default]
    Stopped,
}

#[derive(Debug)]
pub enum MessageThreadMessage {
    ThreadShutdown,
    QueueMessage(String),
}

#[derive(Debug)]
pub struct MessageThreadContext {
    next_insult_message_time_stamp: DateTime<Local>,
    next_announcement_message_time_stamp: DateTime<Local>,
    message_queue: Queue<String>,
    last_message_time: DateTime<Local>,
}

impl Default for MessageThreadContext {
    fn default() -> Self {
        Self {
            next_insult_message_time_stamp: get_local_now(),
            next_announcement_message_time_stamp: get_local_now(),
            message_queue: Queue::new(),
            last_message_time: get_local_now(),
        }
    }
}
pub enum MessageThreadShutdownError {
    ThreadNotRunning,
}

impl MessageThread {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        let (tx, rx) = mpsc::channel::<MessageThreadMessage>(100);
        let thread_handle = tokio::spawn(message_thread_loop(app_handle, rx));

        Self::Running {
            handle: thread_handle,
            sender: tx,
        }
    }

    pub fn shutdown(&mut self) -> Result<(), MessageThreadShutdownError> {
        match self {
            MessageThread::Stopped => Err(MessageThreadShutdownError::ThreadNotRunning),
            MessageThread::Running { sender, handle, .. } => {
                let _ = sender.send(MessageThreadMessage::ThreadShutdown);
                handle.abort();
                *self = MessageThread::Stopped;
                Ok(())
            }
        }
    }

    pub async fn queue_message(&self, message: String) {
        match self {
            MessageThread::Running { sender, .. } => {
                let _ = sender
                    .send(MessageThreadMessage::QueueMessage(message))
                    .await;
            }
            MessageThread::Stopped => (),
        }
    }
}

async fn message_thread_loop(app_handle: tauri::AppHandle, mut rx: Receiver<MessageThreadMessage>) {
    println!("👋 Starting message thread loop.");
    let mut context = MessageThreadContext::default();

    let state = app_handle.state::<Bot>();

    let settings = {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings");
        settings.clone()
    };

    // Adding the minimum time to insults and announcements at the start of the message thread so we don't send messages right away.
    context.next_insult_message_time_stamp =
        get_local_now() + Duration::from_secs(settings.minimum_time_between_insults as u64);
    context.next_announcement_message_time_stamp =
        get_local_now() + Duration::from_secs(settings.minimum_time_between_announcements as u64);

    loop {
        // println!("🔄 Looping message thread.");
        // Receive messages from the channel and handle them.
        let thread_message = match rx.try_recv() {
            Err(err) => match err {
                mpsc::error::TryRecvError::Empty => {
                    // println!("🤷 Message thread channel empty.");
                    None
                }
                mpsc::error::TryRecvError::Disconnected => {
                    println!("🔴 Message thread channel disconnected.");
                    break;
                }
            },
            Ok(thread_message) => Some(thread_message),
        };

        if let Some(thread_message) = thread_message {
            println!("🔍 Received message: {:?}", thread_message);
            match thread_message {
                MessageThreadMessage::ThreadShutdown => {
                    println!("👋 Shutting down message thread.");
                    break;
                }
                MessageThreadMessage::QueueMessage(message) => {
                    println!("📝 Queueing message from channel: {}", message);
                    context.message_queue.enqueue(message);
                }
            }
        }

        let now: DateTime<Local> = get_local_now();

        if settings.enable_insults && now > context.next_insult_message_time_stamp {
            // Run the insult function.
            if let Some(insult) = run_insult(app_handle.clone()) {
                let mut min_time = settings.minimum_time_between_insults;
                let max_time = settings.maximum_time_between_insults;

                // Check to make sure the minimum time is less than the maximum time.
                if min_time > max_time {
                    println!("🔴 Invalid insult timing: min > max. Setting min = max.");
                    min_time = max_time;
                }

                let random_time = rand::thread_rng().gen_range(min_time..=max_time);

                context.next_insult_message_time_stamp =
                    get_local_now() + Duration::from_secs(random_time as u64);

                println!(
                    "📝 Queuing insult message '{}'. Next insult in {} seconds.",
                    insult, random_time
                );

                context.message_queue.enqueue(insult);
            }
        }

        if settings.enable_announcements && now > context.next_announcement_message_time_stamp {
            // Run the announcement function.
            if let Some(announcement) = run_announcement(app_handle.clone()) {
                let mut min_time = settings.minimum_time_between_announcements;
                let max_time = settings.maximum_time_between_announcements;

                // Check to make sure the minimum time is less than the maximum time.
                if min_time > max_time {
                    println!("🔴 Invalid announcement timing: min > max. Setting min = max.");
                    min_time = max_time;
                }

                let random_time = rand::thread_rng().gen_range(min_time..=max_time);

                context.next_announcement_message_time_stamp =
                    get_local_now() + Duration::from_secs(random_time as u64);

                println!(
                    "📝 Queuing announcement message '{}'. Next announcement in {} seconds.",
                    announcement, random_time
                );

                context.message_queue.enqueue(announcement);
            }
        }

        // Make sure we don't send too many messages too quickly.
        if !context.message_queue.is_empty()
            && now
                > context.last_message_time
                    + Duration::from_secs(settings.message_queue_interval as u64)
        {
            // Send a message from the queue.
            let message = context.message_queue.dequeue();
            let _ = say(app_handle.clone(), &message).await;
            // println!("🚀 Sending message: {}", message);
            context.last_message_time = now;
        }

        thread::sleep(Duration::from_secs(1));

        // panic!("Test panic");
    }

    println!("👋 Message thread loop ended.");
}
