use crate::bot::{BotData, User};

#[tauri::command]
pub async fn get_users(state: tauri::State<'_, BotData>) -> Result<Vec<User>, String> {
    let users = state
        .users
        .lock()
        .expect("Failed to get lock for users state.");

    Ok(users.0.clone().into_values().collect())
}
