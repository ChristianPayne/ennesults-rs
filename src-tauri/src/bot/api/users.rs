use tauri::{Emitter, Manager};

use crate::{
    bot::{BotData, User},
    file::write_file,
};

#[tauri::command]
pub async fn get_users(state: tauri::State<'_, BotData>) -> Result<Vec<User>, String> {
    let users = state
        .users
        .lock()
        .expect("Failed to get lock for users state.");

    Ok(users.0.clone().into_values().collect())
}

#[tauri::command]
pub async fn get_active_users(state: tauri::State<'_, BotData>) -> Result<(u32, u32), String> {
    let users = state
        .users
        .lock()
        .expect("Failed to get lock for users state.");

    let total_users = users.0.len() as u32;
    let active_users = users.0.values().filter(|u| u.consented).map(|_| 1).sum();

    Ok((total_users, active_users))
}

#[tauri::command]
pub async fn delete_user(app_handle: tauri::AppHandle, username: String) -> Result<String, String> {
    let state = app_handle.state::<BotData>();
    let mut users = state
        .users
        .lock()
        .expect("Failed to get lock for users state.");

    let _ = users.0.remove(&username);

    if let Err(error) = write_file(&app_handle, "users.json", users.clone()) {
        println!("Failed to write users.json file to disk! {:?}", error);
        let _ = app_handle.emit("error", "Failed to write users.json file to disk!");
    } else {
        let _ = app_handle.emit(
            "users_update",
            users.0.clone().into_values().collect::<Vec<User>>(),
        );
    }

    Ok(username)
}
