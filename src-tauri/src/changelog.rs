use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ChangelogVersionNotes {
    version: &'static str,
    notes: &'static str,
}

const CHANGELOG: [ChangelogVersionNotes; 3] = [
    ChangelogVersionNotes {
        version: "v.1.1.1",
        notes: r"- Fixing updater.",
    },
    ChangelogVersionNotes {
        version: "v.1.1.0",
        notes: r"- Fancy new changelog!
        - You can now edit insults and comebacks after creating them.
        - You can now filter and sort users on the Users page.",
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
