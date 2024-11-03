// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { SerializeRBGColor } from "../../src-tauri/bindings/SerializeRBGColor";

export type Alert = "System" | "Info" | "Warn" | "Error";

export type BotInfo = { channel_name: string, bot_name: string, oauth_token: string, auto_connect_on_startup: boolean, enable_whispers: boolean, enable_insults: boolean, enable_comebacks: boolean, percent_chance_of_comeback: number, };

export type Comeback = { id: number, value: string, };

export type Insult = { id: number, value: string, };

export type TwitchMessage = { username: string, message: string, color: SerializeRBGColor | null, };

export type User = { id: string, username: string, consented: boolean, last_seen: string, };

export type UserLevel = "Viewer" | "Subscriber" | "Vip" | "Moderator" | "Broadcaster";
