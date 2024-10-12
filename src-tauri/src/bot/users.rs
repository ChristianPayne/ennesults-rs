use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
#[serde(default = "Default::default")]
pub struct Users(pub Vec<User>);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "../../src/lib/types.ts")]
pub struct User {
    pub id: String,
    pub username: String,
    pub consented: bool,
}
