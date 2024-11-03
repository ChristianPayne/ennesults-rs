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

    pub enable_comebacks: bool,
    pub percent_chance_of_comeback: u8,
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
            enable_comebacks: true,
            percent_chance_of_comeback: 20,
        }
    }
}
