use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ChangelogVersionNotes {
    version: &'static str,
    notes: &'static str,
}

const CHANGELOG: [ChangelogVersionNotes; 2] = [
    ChangelogVersionNotes {
        version: "v.1.1.0",
        notes: r"- Fancy new changelog!",
    },
    ChangelogVersionNotes {
        version: "v1.0.0",
        notes: r"- Connection to Twitch!
        - Consenting / Unconsenting
        - Command parsing
        - Insults
        - Comebacks
        - Corrections
        - Whispers
        - Templating in replies from the bot.
        - File system integration.
        - Updater",
    },
];

#[tauri::command]
pub fn get_changelog() -> Vec<ChangelogVersionNotes> {
    CHANGELOG.to_vec()
}
