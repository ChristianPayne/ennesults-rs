// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SerializeRBGColor } from "../../src-tauri/bindings/SerializeRBGColor";

export type Alert = "System" | "Info" | "Warn" | "Error";

export type BotInfo = { channel_name: string, bot_name: string, oauth_token: string, auto_connect_on_startup: boolean, enable_whispers: boolean, users_allowed_to_whisper: Array<string>, enable_insults: boolean, time_between_insults: number, minimum_users_in_chat_to_insult: number, enable_comebacks: boolean, percent_chance_of_comeback: number, comeback_exceptions: Array<string>, enable_corrections: boolean, percent_chance_of_correction: number, correction_exceptions: Array<string>, };

export type Comeback = { id: string, value: string, };

export type Insult = { id: string, value: string, };

export type TwitchMessage = { username: string, message: string, color: SerializeRBGColor | null, };

export type User = { id: string, username: string, consented: boolean, last_seen: string, };

export type UserLevel = "Viewer" | "Subscriber" | "Vip" | "Moderator" | "Broadcaster";
