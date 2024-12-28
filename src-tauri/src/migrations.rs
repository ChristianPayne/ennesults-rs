use std::collections::HashMap;

use tauri::{Emitter, Manager};

use crate::bot::{Bot, BotData, Insult, InsultTag, Insults};
use crate::file::{read_json_file, write_file, WriteFileError};

/// Migrations allow us to change the shape of the file system before running the application.  
/// Each migration block should read from the file system and write back to the file system. No state should be touched in any of them as the state has not been managed by Tauri yet.
pub fn run_migrations(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Get migrations file. This holds the function signature name of different migrations.
    let migrations_previously_run =
        read_json_file::<Vec<String>>(&app_handle, "migrations.json").unwrap_or_default();

    let mut migrations_run: Vec<String> = vec![];

    if !migrations_previously_run.contains(&"migrate_insult_tags".to_string()) {
        migrate_insult_tags(app_handle.clone())?;
        migrations_run.push("migrate_insult_tags".to_string());
    }

    // Save the new list of migrations to the file.
    if !migrations_run.is_empty() {
        let mut new_migrations = migrations_previously_run.clone();
        new_migrations.append(&mut migrations_run);

        // Write the migrations data to the file.
        if write_file(&app_handle, "migrations.json", new_migrations).is_err() {
            return Err("Failed to write to migrations file".to_string());
        }
    }

    Ok(())
}

/// 2024-12-28 - Migration to add insult tags to the file system. The default value for tags does not work in our case because we want a default tag of "Insult" to be present on all existing insults. Run the migration once to add the tags. After we run it, we don't want to run it again as someone could remove all tags from an insult and we should not add them back.
pub fn migrate_insult_tags(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut insults_migrated = 0;

    let mut insults = read_json_file::<Insults>(&app_handle, "insults.json").unwrap_or_default();

    // Migration for insults that have no tags associated to them.
    for insult in &mut insults.0 {
        if insult.tags.is_empty() {
            insult.tags.insert(InsultTag::Insult);
            insults_migrated += 1;
        }
    }

    let write_result = write_file::<Insults>(&app_handle, "insults.json", insults);

    if let Some(err) = write_result.err() {
        match err {
            WriteFileError::FailedConvertJSON => {
                return Err("Failed to convert to json.".to_string())
            }
            WriteFileError::FailedCreateFile => return Err("Failed to create file.".to_string()),
            WriteFileError::FailedWriteFile => {
                return Err("Failed to write contents in file.".to_string())
            }
        }
    }

    println!("Insults migrated: {}", insults_migrated);

    Ok(())
}
