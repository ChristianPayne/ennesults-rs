use std::collections::HashMap;

use tauri::{Emitter, Manager};

use crate::bot::{api::save_insults, Bot, BotData, Insult, InsultTag, Insults};
use crate::file::{read_json_file, write_file, WriteFileError};

pub fn run_migrations(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Get migrations file. This holds the function signature name of different migrations.
    let migrations_previously_run =
        read_json_file::<Vec<String>>(&app_handle, "migrations.json").unwrap_or_default();

    let mut migrations_run: Vec<String> = vec![];

    // Check for each migration and run it. No state can be used in these migrations. We run them before we manage state.
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

    // save_insults(app_handle, insults)?;
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
