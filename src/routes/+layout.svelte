<script lang="ts">
  import "../styles.css";
  import { emit, listen } from '@tauri-apps/api/event'
  import { Badge } from "$lib/components/ui/badge";
  import * as Popover from "$lib/components/ui/popover";
  import { ModeWatcher } from "mode-watcher";
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Separator } from "$lib/components/ui/separator";
  import { invoke } from "@tauri-apps/api/core";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Toaster } from "$lib/components/ui/sonner";
  import { toast } from "svelte-sonner";




  let connectionStatus = false;
  let channelName = "none";

  let speakAsEnnesultsMessage = "";
  let speakAsEnnesultsDialog: boolean = false;

  listen('channel_join', (event) => {
    console.log('🪵 ~ unlisten ~ event:', event);
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    channelName = event.payload as string;
    connectionStatus = true;
  })
  listen('channel_part', (event) => {
    console.log('🪵 ~ unlisten ~ event:', event);
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    channelName = event.payload as string;
    connectionStatus = false;
  })

  async function leave_channel () {
    let status = await invoke("leave_channel");
    console.log('🛠 Leave Channel', status);
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
</script>

<div class="flex flex-col h-full">
  <!-- Sidebar -->
  <!-- <div class="bg-primary-600 dark:bg-primary-900 
  w-[250px] min-w-[215px] max-w-[215px] p-2">
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Libero in quis labore perferendis possimus assumenda perspiciatis provident magnam quaerat. Architecto vero iusto dolore deleniti asperiores earum quas culpa delectus odio.
  </div> -->
  <div class="flex flex-col grow">
    <ModeWatcher />
    <!-- Title -->
    <div class="flex justify-between mx-4 my-2">
      <Button variant="ghost" href="/" class="text-2xl font-bold">
        Ennesults
      </Button>
      <div class="flex space-x-2 items-center">
        <Button variant="link" href="/commands">
          Commands
        </Button>
        <Button variant="link" href="/insults">
          Insults
        </Button>
        <Button variant="link" href="/comebacks">
          Comebacks
        </Button>
        <Button variant="link" href="/users">
          Users
        </Button>
        <Button variant="link" href="/settings">
          Settings
        </Button>
        <Button on:click={toggleMode} variant="ghost" size="icon">
          🌙
        </Button>
      </div>
    </div>
    <Separator/>
    <!-- Main Content -->
    <div class="grow mx-4 my-2 overflow-y-scroll">
      <Toaster />
      <slot/>
    </div>
  </div>
  <!-- Footer -->
  <div class="bg-primary-600 py-1 px-2 flex border-t border-primary-800 rounded-t-md">
    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2">
      
      {#if connectionStatus}
        <Dialog.Root bind:open={speakAsEnnesultsDialog}>
          <Dialog.Trigger class="text-sm">Chat</Dialog.Trigger>
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

      <div>
        <Popover.Root>
          <Popover.Trigger>
            <Badge variant="{connectionStatus ? 'secondary' : 'destructive'}">
              {connectionStatus ? channelName : "Disconnected"}
            </Badge>
          </Popover.Trigger>
          
          <Popover.Content>
            { connectionStatus ? `We are connected to ${channelName}.` : "Not connected to a channel." }
            {#if connectionStatus}
              <Button on:click={leave_channel}>
                Leave {channelName}!
              </Button>
            {/if}
          </Popover.Content>
        </Popover.Root>
      </div>
    </div>
  </div>
</div>
