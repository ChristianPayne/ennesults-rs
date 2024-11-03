use crate::bot::{BotData, User};

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
