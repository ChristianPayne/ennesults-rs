<script lang="ts">
  import "../styles.css";
  import { onMount } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';
  import { listen } from '@tauri-apps/api/event'
  import { invoke } from "@tauri-apps/api/core";
  import { ModeWatcher } from "mode-watcher";

  import { Badge } from "$lib/components/ui/badge";
  import * as Popover from "$lib/components/ui/popover";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator";
  import { Toaster } from "$lib/components/ui/sonner";
  import * as Tooltip from "$lib/components/ui/tooltip";
  
  import type { BotInfo } from "$lib/types";
  import SpeakAsEnnesults from "$lib/components/speakAsEnnesults.svelte";
  import NotificationsPanel from "$lib/components/notifications/notificationsPanel.svelte";
  import { alertNotification } from "$lib/components/notifications/notifications";

  let connectionStatus = false;
  let channelName = "";
  let botName = "";
  let tauriVersion = ""
  
  onMount(async () => {
    tauriVersion = await getVersion();
    await getBotInfo();

    listen('bot_info_save', async event => {
      let botInfo = event.payload as BotInfo;
      await getBotInfo(botInfo);
    })

    listen('channel_join', (event) => {
      if(connectionStatus == false) {
        console.log('🛠 channel_join event:', event);
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        channelName = event.payload as string;
        connectionStatus = true;
        alertNotification("System", {
          title: `Connected to ${channelName}!`
        })
      }
    })

    listen('channel_part', (event) => {
      if(connectionStatus == true) {
        console.log('🛠 channel_part event:', event);
        event.payload as string;
        connectionStatus = false;
        alertNotification("Warn", {
          title: `Left ${channelName}!`
        })
      }
    })

    listen('error', (event) => {
      alertNotification("Error", {
        title: event.payload as string
      })
    })

    listen('alert', (event) => {
      alertNotification("Info", {
        title: event.payload as string
      })
    })
  });

  async function getBotInfo(botInfo?: BotInfo) {
    let currentInfo = botInfo ?? await invoke<BotInfo>("get_bot_info").catch(e => console.error(e))
    
    if(currentInfo) {
      channelName = currentInfo.channel_name;
      botName = currentInfo.bot_name;
      if(currentInfo.auto_connect_on_startup) {
        let [ wanted, joined ] = await invoke<[boolean, boolean]>('get_channel_status');
        
        if(wanted == false && joined == false) {
          await connectToChannel()
        }

        if(wanted == true && joined == true) {
          connectionStatus = true;
        }

        if(wanted == true && joined == false) {
          alertNotification("Warn", {
            title: `Still connecting to ${channelName}...`
          })
        }
      }
    }
  }

  async function leave_channel () {
    await invoke<string>("leave_channel").catch(e => {
      alertNotification("Error", {
        title: "Failed to leave channel",
        description: e
      })
    });
  }

  async function connectToChannel () {
    let channel = await invoke<string>("connect_to_channel").catch(err => {
      alertNotification("Error", {
        title: "Failed to connect to channel",
        description: err
      })
    });
    if(channel) {
      alertNotification("System", {
        title: `Connecting to ${channel}...`
      })
    }
  }
</script>

<ModeWatcher />
<Toaster position="bottom-left"/>
<div class="flex flex-col h-full p-2">
  <!-- Title -->
  <div class="flex justify-between mb-2">
    <Button variant="ghost" href="/" class="text-2xl font-bold space-x-2">
      {botName || "Ennesults"}
    </Button>
    <div class="flex space-x-2 items-center">
      <Button variant="ghost" href="/insults">
        Insults
      </Button>
      <Button variant="ghost" href="/comebacks">
        Comebacks
      </Button>
      <Button variant="ghost" href="/users">
        Users
      </Button>
      <Button variant="ghost" href="/settings">
        Settings
      </Button>
      <NotificationsPanel/>
    </div>
  </div>
  <Separator/>
  <!-- Main Content -->
  <div class="grow mx-4 xl:mx-auto xl:w-1/2 my-2 overflow-y-scroll">
    <slot/>
  </div>
  <Separator/>
  <!-- Footer -->
  <div class="p-2 flex align-middle items-center h-8">
    {#if tauriVersion}
    <Tooltip.Root>
      <Tooltip.Trigger>
        <p class="font-light italic text-sm text-gray-400 select-text">ennesults-rs v{tauriVersion}</p>
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Current development build of the bot</p>
      </Tooltip.Content>
    </Tooltip.Root>
    {/if}

    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2 items-center">
      <Popover.Root>
        <Popover.Trigger>
          <Badge variant="{connectionStatus ? 'secondary' : 'destructive'}">
            {#if connectionStatus}
              <p>{channelName}</p>
            {:else}
              <p class="mr-2">Disconnected</p>
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126ZM12 15.75h.007v.008H12v-.008Z" />
              </svg>
            {/if}
          </Badge>
        </Popover.Trigger>
        
        <Popover.Content class="space-y-2 w-96">
          {#if connectionStatus}
            <h1>Channel</h1>
            <p class="flex gap-1">Connected to 
              <a href="https://twitch.tv/{channelName}" target="_blank" class="text-primary flex gap-1 items-center">
                {channelName}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M13.5 6H5.25A2.25 2.25 0 0 0 3 8.25v10.5A2.25 2.25 0 0 0 5.25 21h10.5A2.25 2.25 0 0 0 18 18.75V10.5m-10.5 6L21 3m0 0h-5.25M21 3v5.25" />
                </svg>              
              </a>
            </p>

            <div class="flex gap-2">
              <Button variant="destructive" on:click={leave_channel}>
                Leave Channel
              </Button>
  
              <SpeakAsEnnesults connectionStatus={connectionStatus}/>
            </div>
          {:else}
          <p>Not connected to a channel.</p>
          <Button on:click={connectToChannel}>
            Connect to {channelName || "Channel"}
          </Button>
          {/if}
        </Popover.Content>
      </Popover.Root>
    </div>
  </div>
</div>
