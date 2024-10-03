use crate::bot::{Bot, BotData};

#[tauri::command]
pub async fn connect_to_channel(state: tauri::State<'_, Bot>) -> Result<(), String> {
    let channel_name = state.bot_info.lock().expect("Failed to get lock for bot info").channel_name.clone();
    if !&channel_name.is_empty() {
        if let Some(client) = state.get_client() {
            let channel_status = client
                .get_channel_status(channel_name.clone())
                .await;
            dbg!(channel_status);

            match channel_status {
                (true, false) => {
                    return Err("Already joining a channel.".into());
                },
                (true, true) => {
                    return Err("Already connected to a channel.".into())
                },
                _ => {
                    // join a channel
                    match client.join(channel_name) {
                        Ok(x) => {
                            println!("Connected to channel! {:?}", x);
                            Ok(())
                        }
                        Err(e) => Err(format!("Could not join channel! {}", e)),
                    }
                }
            }
        } else {
            Err("Could not get client.".into())
        }
    } else {
        Err("Channel name not found.".into())
    }
}
