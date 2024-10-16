use tauri::Manager;

use crate::bot::{BotData, Comebacks};
use crate::file::{write_file, WriteFileError};

#[tauri::command]
pub fn save_comebacks(app_handle: tauri::AppHandle, comebacks: Comebacks) -> Result<(), String> {
    let state = app_handle.state::<BotData>();
    *state
        .comebacks
        .lock()
        .expect("Failed to get lock for bot info") = comebacks.clone();

    let write_result = write_file::<Comebacks>(&app_handle, "comebacks.json", comebacks);

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

    Ok(())
}
