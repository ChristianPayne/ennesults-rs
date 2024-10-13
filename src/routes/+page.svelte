<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from "@tauri-apps/api/core"
  import { onDestroy, onMount } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { type TwitchMessage } from '$lib/types';

  const maxChatMessages = 100;

  let messages: TwitchMessage[] = [];
  let unlisten: UnlistenFn;

  let chatElement: Element;

  $: messages, scrollToBottom(chatElement);

  onMount(async () => {
    await getChatMessages();
    console.log("Messages:", messages)
    unlisten = await listen('message', (event: {payload: TwitchMessage}) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      // console.log("event", event.payload)
      messages.push({
        username: event.payload.username,
        message: event.payload.message,
        color: event.payload.color
      });
      messages = messages.slice(-maxChatMessages);

      return event
    });
  })

  onDestroy(() => {
    unlisten();
  })

  const scrollToBottom = async (node: Element) => node?.scroll({ top: node.scrollHeight, behavior: 'instant' })

  async function getChatMessages() {
    let chatMessages = await invoke<TwitchMessage[]>("get_chat_messages")
    messages = chatMessages;
  }

  async function getChatMessagesCount() {
    let count = await invoke("get_chat_messages_count");
    console.log('🪵 ~ getChatMessagesCount ~ count:', count);
    
  }
</script>

<h1>Dashboard</h1>

<div class="md:flex justify-between my-4 gap-4">
  <a href='/insults' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Insults</p>
    <p class="text-4xl font-bold mb-8">50</p>
    <p class="text-muted-foreground">Insults loaded into the bot</p>
  </a>
  <a href='/comebacks' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Comebacks</p>
    <p class="text-4xl font-bold mb-8">12</p>
    <p class="text-muted-foreground">Reactions to users @-ing her</p>
  </a>
  <a href='/users' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Active Users</p>
    <p class="text-4xl font-bold mb-8">42 <span class="text-muted-foreground text-sm">/ 84 Consented</span></p>
    <p class="text-muted-foreground">Users waiting to be insulted</p>
  </a>
</div>

<div class="flex justify-around my-4 gap-2">
  <Button on:click={getChatMessages}>Get Chat Messages</Button>
  <Button on:click={getChatMessagesCount}>Get Chat Messages Count</Button>
</div>

<div class="flex space-x-4">
  <h1>Chat</h1>
  <Button variant="ghost" on:click={() => messages = []}>Clear Chat</Button>
</div>
<div class="overflow-y-scroll h-[600px] select-text border rounded-md p-2" bind:this={chatElement}>
  <ul class="space-y-1">
    {#each messages as message}
      <li><span style="color: rgb({message.color?.[0]},{message.color?.[1]},{message.color?.[2]});" class="text-primary">{message.username}</span>: {message.message}</li>
    {/each}
  </ul>
</div>