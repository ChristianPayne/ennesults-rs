<script lang="ts">
  import Greet from '$lib/Greet.svelte'
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from "@tauri-apps/api/core"
  import { onDestroy, onMount } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import Title from '$lib/Title.svelte';
  import * as Card from "$lib/components/ui/card";

  type MessageDetails = {
    username: string,
    message: string
  }

  let color = "#4E89FF"

  let messages: MessageDetails[] = [];
  let unlisten: UnlistenFn;

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

<Title title="Dashboard"/>

<Button on:click={() => invoke("print_bot_data")}>Print Bot Data</Button>

<div class="md:flex justify-between p-4 gap-4">
  <a href='/insults' class="border rounded-xl p-6 hover:bg-accent">
    <p>Insults</p>
    <p class="text-4xl font-bold mb-8">50</p>
    <p class="text-muted-foreground">Insults loaded into the bot</p>
  </a>
  <a href='/comebacks' class="border rounded-xl p-6 hover:bg-accent">
    <p>Comebacks</p>
    <p class="text-4xl font-bold mb-8">12</p>
    <p class="text-muted-foreground">Reactions to users @-ing her</p>
  </a>
  <a href='/users' class="border rounded-xl p-6 hover:bg-accent">
    <p>Users Consented</p>
    <p class="text-4xl font-bold mb-8">42</p>
    <p class="text-muted-foreground">Users wanting to be insulted</p>
  </a>
</div>

<Title title="Chat"/>
<ul class="space-y-2 select-text">
  {#each messages as message}
    <li><span style="color: {color};">{message.username}</span>: {message.message}</li>
  {/each}
</ul>