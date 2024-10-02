use crate::bot::BotData;

#[tauri::command]
pub async fn print_bot_data(state: tauri::State<'_, BotData>) -> Result<(), &str> {
  dbg!(&state);
  Ok(())
}
