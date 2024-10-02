use crate::bot::BotData;

#[tauri::command]
pub async fn print_bot_data(state: tauri::State<'_, BotData>) -> Result<(), &str> {
  {
    let mut comebacks_state = state.comebacks.lock().unwrap();
  
    // comebacks_state.0.push("Test comeback!".to_string());
  }

  dbg!(&state);
  Ok(())
}
