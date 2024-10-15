<script lang="ts">
  import "../styles.css";
  import { listen } from '@tauri-apps/api/event'
  import { Badge } from "$lib/components/ui/badge";
  import * as Popover from "$lib/components/ui/popover";
  import { ModeWatcher } from "mode-watcher";
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
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  
  let connectionStatus = false;
  let channelName = "";
  
  let speakAsEnnesultsMessage = "";
  let speakAsEnnesultsDialog: boolean = false;

  let tauriVersion = ""
  
  onMount(async () => {
    tauriVersion = await getVersion();
    await getBotInfo();
  });

  async function getBotInfo(botInfo?: BotInfo) {
    let currentInfo = botInfo ?? await invoke<BotInfo>("get_bot_info").catch(e => console.error(e))
    
    if(currentInfo) {
      channelName = currentInfo.channel_name;
      if(currentInfo.auto_connect_on_startup) {
        let [ wanted, joined ] = await invoke<[boolean, boolean]>('get_channel_status');
        
        // if(wanted == false && joined == false) {
        //   await connectToChannel()
        // }

        if(wanted == true && joined == true) {
          connectionStatus = true;
        }

        if(wanted == true && joined == false) {
          toast.warning(`Still connecting to ${channelName}...`)
        }
      }
    }
  }

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
      toast.success(`Connected to ${channelName}!`, {
        dismissable: true
      })
    }
  })

  listen('channel_part', (event) => {
    if(connectionStatus == true) {
      console.log('🛠 channel_part event:', event);
      event.payload as string;
      connectionStatus = false;
      toast.warning(`Left ${channelName}!`)
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
    await invoke<string>("leave_channel").catch(e => {
      toast.error(e)
    });
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

  async function connectToChannel () {
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
      <Sheet.Root>
        <Sheet.Trigger asChild let:builder>
          <Button builders={[builder]} variant="ghost">
            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
              <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
            </svg>
          </Button>
          
        </Sheet.Trigger>
        <Sheet.Content side="right" class="">
          <Sheet.Header>
            <Sheet.Title>Notifications</Sheet.Title>
            <Sheet.Description>
              See notifications from the bot here.
            </Sheet.Description>
          </Sheet.Header>

          <Sheet.Footer>
            <Sheet.Close asChild let:builder>
              <Button builders={[builder]} type="submit">Clear all notifications</Button>
            </Sheet.Close>
          </Sheet.Footer>
        </Sheet.Content>
      </Sheet.Root>
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
  
              <Dialog.Root bind:open={speakAsEnnesultsDialog}>
                <Dialog.Trigger class="text-sm">
                  <Button class="py-0" variant="default">
                    Send Message
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
