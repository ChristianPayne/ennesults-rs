use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ChangelogVersionNotes {
    version: &'static str,
    title: Option<&'static str>,
    notes: &'static str,
}
const CHANGELOG: &[ChangelogVersionNotes] = &[
    ChangelogVersionNotes {
        version: "v1.6.2",
        title: None,
        notes: r"- Bot messages now show up in the chat window.
        - Timestamps are now in the chat window.",
    },
    ChangelogVersionNotes {
        version: "v1.6.1",
        title: None,
        notes: r"- Allow smaller sizes for the overall window size.",
    },
    ChangelogVersionNotes {
        version: "v1.6.0",
        title: Some("Lurk Command"),
        notes: r"- Added a lurk command so users can be insulted when they leave us.
        - Added an insult command to the diagnostics.",
    },
    ChangelogVersionNotes {
        version: "v1.5.0",
        title: Some("Message Thread & Diagnostic Command"),
        notes: r"- Merged the announcements and insults into a single message thread.
        - Added a queue to the new message thread to handle messages better.
        - Added a range (minimum and maximum) to the time between announcements and insults.
        - Added a setting for the message queue interval.
        - Added a diagnostic command to check the status of the bot and run commands.",
    },
    ChangelogVersionNotes {
        version: "v1.4.4",
        title: None,
        notes: r"- Added a {{version}} template option to dynamic data.
        - Added dynamic data replacement to announcements.
        - Fixed an issue where saving users what not saving to local state.",
    },
    ChangelogVersionNotes {
        version: "v1.4.3",
        title: None,
        notes: r"- Fixed a deadlock issue for consent and unconsent getting users.",
    },
    ChangelogVersionNotes {
        version: "v1.4.2",
        title: None,
        notes: r"- Added a random formatting template.",
    },
    ChangelogVersionNotes {
        version: "v1.4.1",
        title: None,
        notes: r"- Fixed deadlock issue for authentication on Windows.
        - New migration script for bot_info rename to settings.",
    },
    ChangelogVersionNotes {
        version: "v1.4.0",
        title: Some("Twitch API Integration"),
        notes: r"- New connection UI on the settings page to authenticate the bot.
        - New UI for the badge at the bottom to show your status.
        - Connection to the Twitch API for future features to query data outside of your chat.",
    },
    ChangelogVersionNotes {
        version: "v1.3.2",
        title: None,
        notes: r"- Fixed locked input fields on new editing panels.
        - New font for Mac and Windows builds.
        - Style changes for Windows.
        - Window now has Ennesults name.
        - Performance optimizations on the chat window.
        - New !version command for moderators.
        - Creator user level added.
        - Simple raid message with name and viewer count.
        - Ennesults now triggers comebacks when using her emote.",
    },
    ChangelogVersionNotes {
        version: "v1.3.1",
        title: None,
        notes: r"- Fixed stale state issue with new editing panels.",
    },
    ChangelogVersionNotes {
        version: "v1.3.0",
        title: Some("Insult Tags"),
        notes: r"- New icon.
        - Settings page formatting changes.
        - New migrations capabilities (insult tags migration).
        - You can now add Insult Tags to insults to use them for certain events.",
    },
    ChangelogVersionNotes {
        version: "v1.2.0",
        title: Some("Announcements"),
        notes: r"- Announcements! Add messages for Ennesults to say over time.
        - Added the ability to get multiple random users per insult.",
    },
    ChangelogVersionNotes {
        version: "v1.1.1",
        title: None,
        notes: r"- Fixed GitHub build action for updater to work successfully.",
    },
    ChangelogVersionNotes {
        version: "v1.1.0",
        title: Some("Editing"),
        notes: r"- Fancy new changelog!
        - You can now edit insults and comebacks after creating them.
        - You can now filter and sort users on the Users page.",
    },
    ChangelogVersionNotes {
        version: "v1.0.0",
        title: Some("MVP"),
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
