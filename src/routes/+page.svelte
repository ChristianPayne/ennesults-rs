<script lang="ts">
  import Greet from '$lib/Greet.svelte'
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from "@tauri-apps/api/core"
  import { Button } from 'flowbite-svelte';
    import { onDestroy, onMount } from 'svelte';

    type MessageDetails = {
      username: string,
      message: string
    }

  let joined_channel = "No joined channel";

  let color = "#4E89FF"

  let messages: MessageDetails[] = [];
  let unlisten: UnlistenFn;

  async function connect_to_channel () {
    
    let status = await invoke("connect_to_channel");
    console.log('🛠 Connect To Channel', status);
  }
  
  async function leave_channel () {
    let status = await invoke("leave_channel");
    console.log('🛠 Leave Channel', status);
  }
  async function print_state () {
    let state = await invoke("print_state");
    console.log('🛠 print_state', state);
  }

  onMount(async () => {
    console.log("Messages:", messages)
    unlisten = await listen('message', (event: {payload: MessageDetails}) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      console.log("event", event.payload, messages)
      messages = [...messages, {
        username: event.payload.username,
        message: event.payload.message
      }]
      return event
    });
  })

  onDestroy(() => {
    unlisten();
  })

  
</script>

<div class="flex space-x-2 mb-4">
  <Button on:click={connect_to_channel}>
    Connect to Ennegineer!
  </Button>
  <Button on:click={leave_channel}>
    Leave Ennegineer!
  </Button>
  <Button on:click={print_state}>
    Print State!
  </Button>
  <p>{joined_channel}</p>
</div>  

<Greet />

<p class="my-4">Twitch Chat</p>
<ul class="space-y-2">
  {#each messages as message}
    <li><span style="color: {color};">{message.username}</span>: {message.message}</li>
  {/each}
</ul>