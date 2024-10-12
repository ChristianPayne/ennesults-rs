use crate::bot::Bot;

#[tauri::command]
pub async fn connect_to_channel(state: tauri::State<'_, Bot>) -> Result<String, String> {
    let channel_name = state.bot_info.lock().expect("Failed to get lock for bot info").channel_name.clone();
    
    if channel_name.is_empty() {
        return Err("Channel name not found.".into())
    }

    let client;
    {
        client = state.client.lock().unwrap().get_client();
    }

    let Some(client) = client else {
        return Err("Could not get client.".into())
    };

    let channel_status = client.get_channel_status(channel_name.clone()).await;

    match channel_status {
        (true, false) => {
            Err("Already joining a channel.".into())
        },
        (true, true) => {
            Err("Already connected to a channel.".into())
        },
        _ => {
            // join a channel
            match client.join(channel_name.clone()) {
                Ok(x) => {
                    println!("Connected to channel! {:?}", x);
                    Ok(channel_name.clone())
                }
                Err(e) => Err(format!("Could not join channel! {}", e)),
            }
        }
    }
}
