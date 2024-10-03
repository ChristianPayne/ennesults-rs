# Ennesults - Rust Edition
Ennesults is a Twitch bot designed to kindly insult random people in chat, originally designed for [Ennegineer](https://www.twitch.tv/ennegineer/).  
This is a rework of the original [Ennesults](https://github.com/ChristianPayne/Ennesults) bot that was written for Node.

# Tauri
This project was intended to get me more familiar with Rust. The original bot works just fine however the tech stack is known very well. The scope of this project allows me to learn new skills as well as provide a better user experience for Enne (UI, hosting the bot herself, etc.).

# Development
## Tech Stack
- Frontend
  - [SvelteKit](https://kit.svelte.dev/)
  - [TailwindCSS](https://tailwindcss.com/)
  - [ShadCN (Svelte)](https://www.shadcn-svelte.com/)
  - [Heroicons](https://heroicons.com/)
- Backend
  - [Tauri](https://v2.tauri.app/)
  - [Twitch IRC](https://docs.rs/twitch-irc/latest/twitch_irc/#)

## File system
Files are being stored in Tauri's `app_data_dir` folder; different for each OS.

## Running the app
From the root directory, use npm to start the front and backend code; `npm run tauri dev`