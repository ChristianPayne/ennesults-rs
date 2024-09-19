use crate::bot;
use crate::bot::Bot;
use crate::config;

// TODO: As it stands, when the connection is first initiated, there is no client so we don't join the channel, we just connect to twitch. Then we need to click it again to join.
#[tauri::command]
pub async fn connect_to_channel(
    app: tauri::AppHandle,
    state: tauri::State<'_, Bot>,
) -> Result<(), &str> {
    match bot::get_client(&state) {
        None => {
            // Add the client to the shared state.
            Bot::connect_to_twitch(app, state);
            Ok(())
        }
        Some(client) => {
            let channel_status = client
                .get_channel_status(config::CHANNEL_NAME.to_owned())
                .await;
            dbg!(channel_status);

            // join a channel
            match client.join(config::CHANNEL_NAME.to_owned()) {
                Ok(_) => {
                    println!("Connected to channel!");
                    Ok(())
                }
                Err(_e) => Err("Could not join channel!"),
            }
        }
    }
}
