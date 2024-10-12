use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[serde(default = "Default::default")]
#[ts(export, export_to="../../../src/lib/types.ts")]
pub struct BotInfo {
    pub channel_name: String,
    pub bot_name: String,
    pub oauth_token: String,
    pub auto_connect_on_startup: bool,

    pub last_comeback_id: u16,
    pub last_insult_id: u16,
}

impl Default for BotInfo {
    fn default() -> Self {
        Self {
            channel_name: "".into(),
            bot_name: "".into(),
            oauth_token: "".into(),
            auto_connect_on_startup: false,
            last_comeback_id: 0,
            last_insult_id: 0,
        }
    }
}