use serde_json::Value;
use tauri::AppHandle;
use ts_rs::TS;

use crate::{date::get_local_now_formatted, twitch::get_broadcaster_id};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct AuthenticationDetails {
    pub access_token: String,
    // pub id_token: String,
    pub client_id: String,
    pub broadcaster_id: String,
    pub login: String,
    pub expires_in: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub enum Authentication {
    Valid {
        details: AuthenticationDetails,
        last_validated: Option<String>,
    },
    /// Probably invalid because the user disconnected the bot from their account.
    Invalid {
        reason: String,
    },
    NotSignedIn,
}

impl Default for Authentication {
    fn default() -> Self {
        Self::NotSignedIn
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthenticationBuilder {
    access_token: Option<String>,
    client_id: Option<String>,
    broadcaster_id: Option<String>,
    login: Option<String>,
    expires_in: Option<i64>,
}

impl AuthenticationBuilder {
    pub fn new() -> AuthenticationBuilder {
        AuthenticationBuilder::default()
    }

    pub fn access_token(&mut self, access_token: String) {
        self.access_token = Some(access_token);
    }
    pub fn client_id(&mut self, client_id: String) {
        self.client_id = Some(client_id);
    }
    pub fn broadcaster_id(&mut self, broadcaster_id: String) {
        self.broadcaster_id = Some(broadcaster_id);
    }
    pub fn login(&mut self, login: String) {
        self.login = Some(login);
    }
    pub fn expires_in(&mut self, expires_in: i64) {
        self.expires_in = Some(expires_in);
    }

    pub fn build(&self) -> Authentication {
        // Check if all values are present.
        if self.access_token.is_some()
            && self.client_id.is_some()
            && self.broadcaster_id.is_some()
            && self.login.is_some()
            && self.expires_in.is_some()
        {
            let copy = self.clone();

            // Can unwrap with confidence because we checked if we have the data first.
            Authentication::Valid {
                details: AuthenticationDetails {
                    access_token: copy.access_token.unwrap(),
                    // id_token: copy.id_token.unwrap(),
                    client_id: copy.client_id.unwrap(),
                    broadcaster_id: copy.broadcaster_id.unwrap(),
                    login: copy.login.unwrap(),
                    expires_in: copy.expires_in.unwrap(),
                },
                last_validated: None,
            }
        } else {
            Authentication::Invalid {
                reason: "Not all data available".to_string(),
            }
        }
    }
}

#[derive(Debug)]
pub enum AuthenticationError {
    ParsingError(String),
}

/// Take in details to validate, check them against the Twitch Validate API and return the details back or error.
pub async fn validate_auth(
    app_handle: AppHandle,
    access_token: String,
) -> Result<Authentication, AuthenticationError> {
    println!("Validating details through Twitch");
    // Make a request to the validation endpoint.
    let client = reqwest::Client::new();
    let resp = client
        .get("https://id.twitch.tv/oauth2/validate")
        .header("Authorization", format!("OAuth {}", &access_token))
        .send()
        .await
        .map_err(|e| AuthenticationError::ParsingError(e.to_string()))?
        .text()
        .await
        .map_err(|e| AuthenticationError::ParsingError(e.to_string()))?;

    // Make sure we can parse the JSON.
    let resp: Value = serde_json::from_str(&resp)
        .map_err(|e| AuthenticationError::ParsingError(e.to_string()))?;

    // Check what values are in the response.
    match (
        // Present on 200
        resp["login"].clone(),
        resp["expires_in"].clone(),
        resp["client_id"].clone(),
        // Present on 401
        resp["message"].clone(),
    ) {
        // All 200 values are present.
        (Value::String(login), Value::Number(expires_in), Value::String(client_id), _) => {
            // Make sure that we can parse the data coming back from Twitch.
            let Some(expires_in) = expires_in.as_i64() else {
                return Err(AuthenticationError::ParsingError(
                    "Failed to convert expires_in value".to_string(),
                ));
            };

            let broadcaster_id =
                get_broadcaster_id(app_handle.clone(), client_id.clone(), access_token.clone())
                    .await
                    .map_err(|e| AuthenticationError::ParsingError(e))?;

            Ok(Authentication::Valid {
                details: AuthenticationDetails {
                    access_token,
                    broadcaster_id,
                    client_id,
                    // id_token: authentication_details.id_token,
                    login,
                    expires_in,
                },
                last_validated: Some(get_local_now_formatted()),
            })
        }
        // No 200 values are present but we have a 401 message.
        (Value::Null, Value::Null, Value::Null, Value::String(message)) => {
            Ok(Authentication::Invalid { reason: message })
        }
        // Not something we were expecting.
        _ => Err(AuthenticationError::ParsingError(
            "Failed to parse response contents".to_string(),
        )),
    }
}

pub mod api {
    use std::{collections::HashMap, sync::Mutex};

    use serde_json::Value;
    use tauri::{AppHandle, Emitter, Manager, Url};
    use url_builder::URLBuilder;

    use crate::{
        bot::{
            api::{connect_to_twitch, disconnect_from_twitch},
            Bot,
        },
        file::{write_file, WriteFileError},
    };

    use super::{validate_auth, Authentication, AuthenticationBuilder};

    /// Opens a new window from Ennesults to log in the user.
    #[tauri::command]
    pub fn open_auth_window(app_handle: AppHandle) -> Result<(), String> {
        if !app_handle.manage(AuthenticationBuilder::new()) {
            return Err("Authentication Builder state is already being managed.".to_string());
        }

        let client_id = "nbdppbmm4iicute0sl1cj663xyvbi4".to_string();

        let mut ub = URLBuilder::new();
        ub.set_protocol("https")
            .set_host("id.twitch.tv/oauth2/authorize")
            .add_param("response_type", "token")
            .add_param("client_id", &client_id)
            .add_param("redirect_uri", format!("http://localhost:{}", 4500).as_str())
            .add_param("scope", "channel:bot moderator:read:chatters moderator:read:followers moderator:read:shoutouts moderator:manage:shoutouts chat:read whispers:read user:write:chat chat:edit".replace(":", "%3A").replace(" ", "%20").as_str())
            .add_param("state", "ennesults-rocks");

        let url = ub.build();

        let window_result = tauri::WebviewWindowBuilder::new(
            &app_handle,
            "auth",
            tauri::WebviewUrl::App(url.into()),
        )
        .title("Ennesults Authentication")
        .incognito(true)
        .build();

        Ok(())
    }

    /// Handles the redirect URL from twitch with the authenticated details inside.
    #[tauri::command]
    pub async fn decode_auth_redirect(
        app_handle: AppHandle,
        url: String,
    ) -> Result<Authentication, String> {
        println!("URL from Twitch redirect: {}", url);

        let url = url.replace("#", "?");
        let parsed_url = Url::parse(&url).unwrap();
        let hash_query: HashMap<_, _> = parsed_url.query_pairs().into_owned().collect();

        dbg!(&hash_query);

        let Some(access_token) = hash_query.get("access_token") else {
            // Send an emit to the front end that we didn't get the access token.
            let _ = app_handle.emit("error", "Failed to decode access token!");
            return Err("Failed to decode access token!".to_string());
        };

        // Save the access token.
        println!("Successfully received access token: {}", access_token);

        // let Some(id_token) = hash_query.get("id_token") else {
        //     // Send an emit to the front end that we didn't get the access token.
        //     let _ = app_handle.emit("error", "Failed to decode id token!");
        //     return Err("Failed to decode id token!".to_string());
        // };

        // auth_builder = auth_builder.id_token(id_token.clone());

        // Validating auth allows us to get all the pieces of data we need.
        let Ok(auth_validation) = validate_auth(app_handle.clone(), access_token.clone()).await
        else {
            return Err("Failed to validate auth during auth decoding".to_string());
        };

        dbg!(&auth_validation);

        if let Err(err) =
            write_file::<Authentication>(&app_handle, "auth.json", auth_validation.clone())
        {
            return match err {
                WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
                WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
                WriteFileError::FailedWriteFile => {
                    Err("Failed to write contents in file.".to_string())
                }
            };
        }

        let bot = app_handle.state::<Bot>();

        {
            let mut auth = bot.auth.lock().expect("Failed to get lock for Auth");
            *auth = auth_validation.clone();
        }

        connect_to_twitch(app_handle.clone());

        Ok(auth_validation)
    }

    #[tauri::command]
    pub fn sign_out_of_twitch(app_handle: AppHandle) -> Result<Authentication, String> {
        let bot = app_handle.state::<Bot>();

        let write_result = write_file::<Value>(&app_handle, "auth.json", Value::Null);

        if let Some(err) = write_result.err() {
            return match err {
                WriteFileError::FailedConvertJSON => Err("Failed to convert to json.".to_string()),
                WriteFileError::FailedCreateFile => Err("Failed to create file.".to_string()),
                WriteFileError::FailedWriteFile => {
                    Err("Failed to write contents in file.".to_string())
                }
            };
        }

        // Disconnect from Twitch.
        disconnect_from_twitch(app_handle.clone());

        {
            let mut auth = bot.auth.lock().expect("Failed to get lock for auth");
            *auth = Authentication::NotSignedIn;
        }

        Ok(Authentication::NotSignedIn)
    }
}
