use rand::seq::SliceRandom;
use tauri::{AppHandle, Manager};
use ts_rs::TS;

use crate::bot::{
    users::{get_random_user, User, Users},
    Bot,
};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Announcements {
    pub announcements: Vec<Announcement>,
    pub next_announcement_index: usize,
}

impl Announcements {
    pub fn from(announcements: Vec<Announcement>) -> Self {
        Self {
            announcements,
            next_announcement_index: 0,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct Announcement {
    pub id: String,
    pub value: String,
}

pub fn run_announcement(app_handle: AppHandle) -> Option<String> {
    let state = app_handle.state::<Bot>();
    let randomize_announcements = {
        let settings = state
            .settings
            .lock()
            .expect("Failed to get lock for settings");

        settings.randomize_announcements
    };

    let announcements = {
        let announcements = state
            .bot_data
            .announcements
            .lock()
            .expect("Failed to get lock for insults.");

        announcements.announcements.clone()
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
                let mut existing_announcements = state
                    .bot_data
                    .announcements
                    .lock()
                    .expect("Failed to get lock for insults.");

                let index = existing_announcements.next_announcement_index;
                let chosen_announcement = &announcements[index];

                existing_announcements.next_announcement_index = (index + 1) % announcements.len();

                Some(chosen_announcement)
            }
        }
    };

    match announcement {
        Some(announcement) => format_announcement(app_handle.clone(), announcement, None),
        None => {
            println!("Could not get an announcement to say.");
            None
        }
    }
}

pub fn format_announcement(
    app_handle: tauri::AppHandle,
    announcement: &Announcement,
    user_pool: Option<Vec<User>>,
) -> Option<String> {
    let state = app_handle.state::<Bot>();
    let mut formatted_message = announcement.value.clone();

    // Format for any streamer tags.
    if formatted_message.contains("{{streamer}}") {
        let channel_name = {
            let state = app_handle.state::<Bot>();
            state.get_channel_name()
        };

        formatted_message = formatted_message.replace("{{streamer}}", channel_name.as_str())
    }

    // Format for any version tags.
    if formatted_message.contains("{{version}}") {
        let version = format!("v{}", app_handle.package_info().version.clone());

        formatted_message = formatted_message.replace("{{version}}", &version)
    }

    if formatted_message.contains("{{random}}") {
        let mut users: Users = {
            match user_pool {
                None => state.bot_data.get_users(),
                Some(users) => Users::from(users),
            }
        };

        // Format for any random tags.
        while formatted_message.contains("{{random}}") {
            let random_user = get_random_user(
                app_handle.clone(),
                !announcement.value.contains("{{streamer}}"),
                &users,
                true,
            )
            .cloned();

            match random_user {
                Some(user) => {
                    // Remove the user so that we don't pick it again if we go around again.
                    users.0.remove(&user.username);

                    // Replace just the first instance of the tag.
                    formatted_message =
                        formatted_message.replacen("{{random}}", user.username.as_str(), 1);
                }
                None => {
                    println!(
                        "🟡 Not enough random consented users available to format announcement."
                    );
                    return None;
                }
            }
        }
    }

    Some(formatted_message)
}

pub mod api {
    use surrealdb::opt::Resource;
    use tauri::{Emitter, Manager};

    use crate::{
        bot::{
            announcements::{self, Announcement},
            Bot,
        },
        helpers::file::{write_file, WriteFileError},
    };

    #[tauri::command]
    pub async fn get_announcements(
        app_handle: tauri::AppHandle,
    ) -> Result<Vec<Announcement>, String> {
        // Get from SurrealDB
        let db = match crate::database::connect_to_database(&app_handle).await {
            Ok(db) => db,
            Err(e) => return Err(format!("Failed to connect to database: {:?}", e)),
        };

        // Left off here trying to get all announcements from the database. 
        // The problems is that surreal can't convert to a vec of announcements.
        // There is garbage in their "thing" type and it is a hassle to convert.
        // https://github.com/surrealdb/surrealdb/issues/5794
        let announcements = db
            .select(Resource::from("announcements"))
            .await
            .map_err(|e| format!("Failed to query announcements: {:?}", e))?;

        dbg!(&announcements);

        Ok(announcements)
    }

    #[tauri::command]
    pub async fn update_announcement(
        app_handle: tauri::AppHandle,
        announcement: Announcement,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        {
            let mut announcements = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements.")
                .clone();

            match announcements
                .announcements
                .iter_mut()
                .find(|i| i.id == announcement.id)
            {
                Some(announcement_in_db) => {
                    announcement_in_db.value = announcement.value.clone();
                }
                None => return Err("Failed to find announcement in database".to_string()),
            }

            // Update in-memory state
            let mut announcements_state = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements.");
            announcements_state.announcements = announcements.announcements.clone();
        } // Drop mutex guards

        // Update in SurrealDB
        let db = match crate::database::connect_to_database(&app_handle).await {
            Ok(db) => db,
            Err(e) => return Err(format!("Failed to connect to database: {:?}", e)),
        };

        // Use raw query to avoid serialization issues
        let query = format!(
            "UPDATE announcements:{} CONTENT {{ id: '{}', value: '{}' }}",
            announcement.id, announcement.id, announcement.value
        );

        match db.query(query).await {
            Ok(_) => println!("✅ Updated announcement: {}", announcement.id),
            Err(e) => return Err(format!("Failed to update announcement: {:?}", e)),
        }

        let _ = app_handle.emit("announcements_update", announcement.clone());

        Ok(())
    }

    #[tauri::command]
    pub async fn save_announcement(
        app_handle: tauri::AppHandle,
        announcement: Announcement,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();

        // Save to SurrealDB
        let db = match crate::database::connect_to_database(&app_handle).await {
            Ok(db) => db,
            Err(e) => return Err(format!("Failed to connect to database: {:?}", e)),
        };

        // Use raw query to avoid serialization issues
        let query = format!(
            "CREATE announcements:{} CONTENT {{ id: '{}', value: '{}' }}",
            announcement.id, announcement.id, announcement.value
        );

        match db.query(query).await {
            Ok(_) => println!("✅ Saved announcement: {}", announcement.id),
            Err(e) => {
                println!("Failed to save announcement {:?}", &e);
                return Err(format!(
                    "Failed to save announcement {}: {:?}",
                    announcement.id, e
                ));
            }
        }

        {
            let mut announcements_state = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for settings");

            // Check if announcement already exists
            if let Some(existing) = announcements_state
                .announcements
                .iter_mut()
                .find(|a| a.id == announcement.id)
            {
                // Update existing
                existing.value = announcement.value.clone();
            } else {
                // Add new
                announcements_state.announcements.push(announcement.clone());
            }

            let _ = app_handle.emit("announcements_update", announcements_state.clone());
        } // Drop the mutex guard here

        Ok(())
    }

    #[tauri::command]
    pub async fn delete_announcement(
        app_handle: tauri::AppHandle,
        announcement_id: String,
    ) -> Result<(), String> {
        let state = app_handle.state::<Bot>();
        {
            let mut announcements = state
                .bot_data
                .announcements
                .lock()
                .expect("Failed to get lock for announcements");

            match announcements
                .announcements
                .iter()
                .position(|announcement| announcement.id == announcement_id)
            {
                None => return Err("Could not find announcement.".to_string()),
                Some(index) => {
                    announcements.announcements.remove(index);
                }
            }
        } // Drop mutex guard

        // Delete from SurrealDB
        let db = match crate::database::connect_to_database(&app_handle).await {
            Ok(db) => db,
            Err(e) => return Err(format!("Failed to connect to database: {:?}", e)),
        };

        match db
            .delete::<Option<Announcement>>(("announcements", &announcement_id))
            .await
        {
            Ok(_) => println!("✅ Deleted announcement: {}", announcement_id),
            Err(e) => return Err(format!("Failed to delete announcement: {:?}", e)),
        }

        let _ = app_handle.emit("announcements_update", announcement_id);

        Ok(())
    }
}
