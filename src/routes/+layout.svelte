<script lang="ts">
  import "../styles.css";
  import { listen } from '@tauri-apps/api/event'
  import { Badge } from "$lib/components/ui/badge";
  import * as Popover from "$lib/components/ui/popover";
  import { ModeWatcher } from "mode-watcher";
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator";
  import { invoke } from "@tauri-apps/api/core";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Toaster } from "$lib/components/ui/sonner";
  import { toast } from "svelte-sonner";
  import { getVersion } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';
  import * as Tooltip from "$lib/components/ui/tooltip";
  import type { BotInfo } from "$lib/types";
  
  let connectionStatus = false;
  let channelName = "";
  
  let speakAsEnnesultsMessage = "";
  let speakAsEnnesultsDialog: boolean = false;

  let tauriVersion = ""
  
  onMount(async () => {
    tauriVersion = await getVersion();
    let botInfo = await invoke<BotInfo>("get_bot_info").catch(e => console.error(e))
    if(botInfo) {
      channelName = botInfo.channel_name;
      if(botInfo.auto_connect_on_startup) {
        let [ wanted, joined ] = await invoke<[boolean, boolean]>('get_channel_status');
        
        if(wanted == false && joined == false) {
          await connect_to_channel()
        }

        if(wanted == true && joined == true) {
          connectionStatus = true;
        }

        if(wanted == true && joined == false) {
          toast.warning(`Still connecting to ${channelName}...`)
        }
      }
    }
  });

  listen('channel_join', (event) => {
    if(connectionStatus == false) {
      console.log('🛠 channel_join event:', event);
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      channelName = event.payload as string;
      connectionStatus = true;
      toast.success(`Connected to ${channelName}!`, {
        dismissable: true
      })
    }
  })

  listen('channel_part', (event) => {
    if(connectionStatus == true) {
      console.log('🛠 channel_part event:', event);
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      leave_channel()
    }
  })

  listen('error', (event) => {
    toast.error(event.payload as string, {
      duration: 10000
    })
  })

  listen('alert', (event) => {
    toast.info(event.payload as string)
  })

  async function leave_channel () {
    let status = await invoke<string>("leave_channel").catch(e => {
      toast.error(e)
      return;
    });
    if(status) {
      channelName = status as string;
      connectionStatus = false;
      toast.warning(`Left ${channelName}!`)
    }
  }

  async function speakAsEnnesults () {
    if(speakAsEnnesultsMessage === "") return;
    if(connectionStatus === false) return;

    let result = await invoke("say", {message: speakAsEnnesultsMessage}).catch(e => {
      toast.error("Something went wrong!", {
        description: "Failed to send chat message!" + e,
      })
    });
    if(result === true) {
      speakAsEnnesultsMessage = ""
      speakAsEnnesultsDialog = false;
    }
  }

  async function connect_to_channel () {
    let channel = await invoke<string>("connect_to_channel").catch(err => {
      toast.error(err);
    });
    if(channel) {
      toast.info(`Connecting to ${channel}...`,)
    }

    console.log('🛠 Connect To Channel', channel);
  }
</script>

<ModeWatcher />
<Toaster position="bottom-left"/>
<div class="flex flex-col h-full p-2">
  <!-- Title -->
  <div class="flex justify-between mb-2">
    <Button variant="ghost" href="/" class="text-2xl font-bold space-x-2">
      Ennesults
    </Button>
    <div class="flex space-x-2 items-center">
      <Button variant="ghost" href="/commands">
        Commands
      </Button>
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
      <Button on:click={toggleMode} variant="ghost" size="icon">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
        </svg>
      </Button>
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
        <p class="font-light italic text-sm text-gray-400">ennesults-rs <span class="select-text">v{tauriVersion}</span></p>
      </Tooltip.Trigger>
      <Tooltip.Content>
        <p>Current development build of the bot</p>
      </Tooltip.Content>
    </Tooltip.Root>
    {/if}

    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2 items-center">
      
      {#if connectionStatus}
        <Dialog.Root bind:open={speakAsEnnesultsDialog}>
          <Dialog.Trigger class="text-sm align-middle">
            <Button class="py-0" variant="ghost" size="sm">
              Chat
            </Button>
          </Dialog.Trigger>
          <Dialog.Content>
            <Dialog.Header> 
              <Dialog.Title>Make Ennesults speak in chat!</Dialog.Title>
              <Dialog.Description>
                <div class="grid w-full gap-1.5">
                  <Textarea bind:value={speakAsEnnesultsMessage} placeholder="Type your message here." />
                  <Button on:click={speakAsEnnesults}>Send</Button>
                </div>
              </Dialog.Description>
            </Dialog.Header>
          </Dialog.Content>
        </Dialog.Root>
      {/if}

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
        
        <Popover.Content class="space-y-2">
          {#if connectionStatus}
            <p>Connected to {channelName}.</p>
            <Button variant="destructive" on:click={leave_channel}>
              Leave Channel
            </Button>
          {:else}
          <p>Not connected to a channel.</p>
          <Button on:click={connect_to_channel}>
            Connect to {channelName || "Channel"}
          </Button>
          {/if}
        </Popover.Content>
      </Popover.Root>
    </div>
  </div>
</div>
