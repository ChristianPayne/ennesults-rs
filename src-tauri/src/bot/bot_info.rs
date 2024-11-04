use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[serde(default = "Default::default")]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct BotInfo {
    pub channel_name: String,
    pub bot_name: String,
    pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub enable_whispers: bool,

    pub enable_insults: bool,
    pub minimum_users_in_chat_to_insult: u32,

    pub enable_comebacks: bool,
    pub percent_chance_of_comeback: u32,
    pub comeback_exceptions: Vec<String>,

    pub enable_corrections: bool,
    pub correction_exceptions: Vec<String>,
}

impl Default for BotInfo {
    fn default() -> Self {
        Self {
            channel_name: "".into(),
            bot_name: "".into(),
            oauth_token: "".into(),
            auto_connect_on_startup: false,
            enable_whispers: true,
            enable_insults: true,
            minimum_users_in_chat_to_insult: 1,
            enable_comebacks: true,
            percent_chance_of_comeback: 20,
            comeback_exceptions: vec![],
            enable_corrections: true,
            correction_exceptions: vec![],
        }
    }
}
