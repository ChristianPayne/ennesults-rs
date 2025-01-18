<script lang="ts">
  import "../styles.css";
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { ModeWatcher } from "mode-watcher";

  import { Badge } from "$lib/components/ui/badge";
  import * as Popover from "$lib/components/ui/popover";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator";
  import { Toaster } from "$lib/components/ui/sonner";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import * as Dialog from "$lib/components/ui/dialog";

  import type { Authentication, BotInfo } from "$lib/types";
  import SpeakAsEnnesults from "$lib/components/speakAsEnnesults.svelte";
  import NotificationsPanel from "$lib/components/notifications/notificationsPanel.svelte";
  import { alertNotification } from "$lib/components/notifications/notifications";

  import { fade } from "svelte/transition";
  import Changelog from "$lib/components/Changelog.svelte";

  export let data;

  let authentication: Authentication;

  let connectionStatus = false;
  let channelName = "";
  let botName = "";
  let tauriVersion = "";

  let changelog;

  onMount(async () => {
    tauriVersion = await getVersion();
    changelog = await invoke("get_changelog");
    await invoke<Authentication>("connect_to_twitch").catch((e: string) => {
      alertNotification("Error", {
        title: "Error",
        description: e,
      });
    });

    authentication = await invoke<Authentication>("get_auth_status");
    await getBotInfo();

    listen("bot_info_save", async (event) => {
      let botInfo = event.payload as BotInfo;
      await getBotInfo(botInfo);
    });

    listen("auth", async (event) => {
      authentication = event.payload as Authentication;
    });

    listen("channel_join", (event) => {
      if (connectionStatus == false) {
        console.log("🛠 channel_join event:", event);
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        channelName = event.payload as string;
        connectionStatus = true;
        alertNotification("System", {
          title: `Connected to ${channelName}!`,
        });
      }
    });

    listen("channel_part", (event) => {
      if (connectionStatus == true) {
        console.log("🛠 channel_part event:", event);
        event.payload as string;
        connectionStatus = false;
        alertNotification("Warn", {
          title: `Left ${channelName}!`,
        });
      }
    });

    listen("error", (event) => {
      alertNotification("Error", {
        title: event.payload as string,
      });
    });

    listen("alert", (event) => {
      alertNotification("Info", {
        title: event.payload as string,
      });
    });
  });

  async function getBotInfo(botInfo?: BotInfo) {
    let currentInfo =
      botInfo ??
      (await invoke<BotInfo>("get_bot_info").catch((e) => console.error(e)));

    if (!currentInfo) return;

    channelName = currentInfo.channel_name;
    botName = authentication["Valid"]?.details?.login ?? "Ennesults";

    if (!currentInfo.auto_connect_on_startup) return;

    try {
      let [wanted, joined] =
        await invoke<[boolean, boolean]>("get_channel_status");

      if (wanted === undefined || joined === undefined) return;

      if (wanted == false && joined == false) {
        await connectToChannel();
      }

      if (wanted == true && joined == true) {
        connectionStatus = true;
      }

      if (wanted == true && joined == false) {
        alertNotification("Warn", {
          title: `Still connecting to ${channelName}...`,
        });
      }
    } catch (e) {
      console.log(e);
    }
  }

  async function leave_channel() {
    await invoke<string>("leave_channel").catch((e) => {
      alertNotification("Error", {
        title: "Failed to leave channel",
        description: e,
      });
    });
  }

  async function connectToChannel() {
    let channel = await invoke<string>("connect_to_channel").catch((err) => {
      alertNotification("Error", {
        title: "Failed to connect to channel",
        description: err,
      });
    });
    if (channel) {
      alertNotification("System", {
        title: `Connecting to ${channel}...`,
      });
    }
  }
</script>

<ModeWatcher />
<Toaster position="bottom-left" />
<div class="flex flex-col h-full">
  <!-- Title -->
  <div class="flex flex-col sm:flex-row justify-between mb-2 p-2">
    <Button variant="ghost" href="/" class="text-2xl font-bold space-x-2">
      Ennesults
    </Button>
    <div class="sm:flex sm:space-x-2 items-center">
      <Button variant="ghost" href="/announcements">Announcements</Button>
      <Button variant="ghost" href="/insults">Insults</Button>
      <Button variant="ghost" href="/comebacks">Comebacks</Button>
      <Button variant="ghost" href="/users">Users</Button>
      <Button variant="ghost" href="/settings">Settings</Button>
      <NotificationsPanel />
    </div>
  </div>
  <Separator />
  <!-- Main Content -->
  <div class="grow px-4 xl:mx-auto xl:w-1/2 my-2 overflow-y-scroll">
    {#key data.pathname}
      <div in:fade={{ duration: 100, delay: 100 }} out:fade={{ duration: 100 }}>
        <slot />
      </div>
    {/key}
  </div>
  <Separator />
  <!-- Footer -->
  <div class="p-2 flex align-middle items-center h-8">
    {#if tauriVersion}
      <Tooltip.Root>
        <Tooltip.Trigger>
          <Dialog.Root>
            <Dialog.Trigger>
              <p class="font-light italic text-sm text-gray-400 select-text">
                ennesults-rs v{tauriVersion}
              </p>
            </Dialog.Trigger>
            <Dialog.Content>
              <Changelog {changelog} />
            </Dialog.Content>
          </Dialog.Root>
        </Tooltip.Trigger>
        <Tooltip.Content>
          <p>Current development build of the bot</p>
        </Tooltip.Content>
      </Tooltip.Root>
    {/if}

    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2 items-center">
      {#if authentication}
        <Popover.Root>
          <Popover.Trigger>
            <Badge variant={connectionStatus ? "secondary" : "destructive"}>
              {#if authentication["Valid"]}
                {channelName || "Configure channel name"}
              {:else if authentication["Invalid"]}
                Invalid Authentication
              {:else}
                Not Signed In
              {/if}
            </Badge>
          </Popover.Trigger>

          <Popover.Content class="space-y-2 w-96">
            {#if authentication["Valid"]}
              {#if channelName}
                {#if connectionStatus}
                  <h1>Channel</h1>
                  <p class="flex gap-1">
                    Connected to
                    <a
                      href="https://twitch.tv/{channelName}"
                      target="_blank"
                      class="text-primary flex gap-1 items-center"
                    >
                      {channelName}
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                        class="size-5"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25"
                        />
                      </svg>
                    </a>
                  </p>

                  <div class="flex gap-2">
                    <Button variant="destructive" on:click={leave_channel}>
                      Leave Channel
                    </Button>

                    <SpeakAsEnnesults {connectionStatus} />
                  </div>
                {:else}
                  <p>Not connected to a channel.</p>
                  <Button on:click={connectToChannel}>
                    Connect to {channelName || "Channel"}
                  </Button>
                {/if}
              {:else}
                <h1>Channel name not configured</h1>
                <p>Set up what channel you want to the bot to connect to.</p>
                <Button variant="default" href="settings">Go to Settings</Button
                >
              {/if}
            {:else if authentication["Invalid"]}
              <h1>Twitch Authentication is Invalid!</h1>
              <p>Please log in again using the settings page.</p>
              <Button variant="default" href="settings">Go to Settings</Button>
            {:else if authentication === "NotSignedIn"}
              <h1>Not Signed In</h1>
              <p>Connecting to Twitch can be done in the settings page.</p>
              <Button variant="default" href="settings">Go to Settings</Button>
            {/if}
          </Popover.Content>
        </Popover.Root>
      {/if}
    </div>
  </div>
</div>
