use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::bot::Bot;

/// Gets the id of the channel that we are wanting to join.
pub async fn get_broadcaster_id(
    app_handle: AppHandle,
    client_id: String,
    access_token: String,
) -> Result<String, String> {
    let bot = app_handle.state::<Bot>();
    let channel_name = bot.get_channel_name();

    let client = reqwest::Client::new();

    let resp = client
        .get(format!(
            "https://api.twitch.tv/helix/users?login={}",
            channel_name
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .send()
        .await
        .map_err(|e| format!("Errored on send: {}", e))?
        .text()
        .await
        .map_err(|e| format!("Errored on text(): {}", e))?;

    let resp: Value =
        serde_json::from_str(&resp).map_err(|e| format!("Errored on serde: {}", e))?;

    let Value::Array(vec) = &resp["data"] else {
        return Err("Data received from Twitch is not an array".to_string());
    };

    if vec.len() != 1 {
        return Err(format!(
            "Twitch array is not the correct length: {}",
            vec.len()
        ));
    }

    let Value::Object(data) = &vec[0] else {
        return Err("Twitch data is not an object".to_string());
    };

    let Some(id_value) = data.get("id") else {
        return Err("Could not find id in Twitch Data.".to_string());
    };

    match id_value {
        Value::String(id) => Ok(id.clone()),
        _ => Err("Id is not a string value in Twitch Data.".to_string()),
    }
}
