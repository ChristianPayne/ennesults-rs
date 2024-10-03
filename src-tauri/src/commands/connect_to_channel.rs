use crate::bot::{Bot, BotData};

#[tauri::command]
pub async fn connect_to_channel(state: tauri::State<'_, Bot>) -> Result<String, String> {
    let channel_name = state.bot_info.lock().expect("Failed to get lock for bot info").channel_name.clone();
    match &channel_name.is_empty() {
        false => {
            match state.get_client() {
                None => Err("Could not get client.".into()),
                Some(client) => {
                    let channel_status = client.get_channel_status(channel_name.clone()).await;
                    // dbg!(channel_status);
        
                    match channel_status {
                        (true, false) => {
                            return Err("Already joining a channel.".into());
                        },
                        (true, true) => {
                            return Err("Already connected to a channel.".into())
                        },
                        _ => {
                            // join a channel
                            match client.join(channel_name.clone()) {
                                Ok(x) => {
                                    println!("Connected to channel! {:?}", x);
                                    Ok(channel_name)
                                }
                                Err(e) => Err(format!("Could not join channel! {}", e)),
                            }
                        }
                    }
                }
            }
        },
        true => Err("Channel name not found.".into())
    }
}
