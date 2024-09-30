use twitch_irc::message::PrivmsgMessage;

use crate::bot;
use crate::bot::Bot;
use crate::config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn get_channel_name(state: tauri::State<'_, Bot>) -> Result<String, String> {
  Ok(state.channel_name.clone())
}
