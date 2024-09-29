use crate::bot;
use crate::bot::Bot;
use crate::config;

// TODO: As it stands, when the connection is first initiated, there is no client so we don't join the channel, we just connect to twitch. Then we need to click it again to join.
#[tauri::command]
pub async fn connect_to_channel(state: tauri::State<'_, Bot>) -> Result<(), &str> {
    if !&state.channel_name.is_empty() {
        if let Some(client) = state.get_client() {
            let channel_status = client
                .get_channel_status(state.channel_name.clone())
                .await;
            dbg!(channel_status);
    
            // join a channel
            match client.join(config::CHANNEL_NAME.to_owned()) {
                Ok(_) => {
                    println!("Connected to channel!");
                    Ok(())
                }
                Err(_) => Err("Could not join channel!"),
            }
        } else {
            Err("Could not get client.")
        }
    } else {
        Err("Channel name not found.")
    }
}
