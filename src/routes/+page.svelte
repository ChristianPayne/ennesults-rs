<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from "@tauri-apps/api/core"
  import { onDestroy, onMount } from 'svelte';
  import { Button } from "$lib/components/ui/button";
  import { type TwitchMessage } from '$lib/types';
    import NumberTicker from '$lib/components/NumberTicker.svelte';

  const maxChatMessages = 100;

  let messages: TwitchMessage[] = [];
  let unlistenMessage: UnlistenFn;
  let unlistenActiveUsers: UnlistenFn;

  let chatElement: Element;

  let activeUserStats: [totalUsers: number, activeUsers: number] = [0,0];

  let insultCount = 0;
  let comebacksCount = 0;

  $: totalUsers = activeUserStats[0];
  $: activeUsers = activeUserStats[1];

  $: messages, scrollToBottom(chatElement);

  onMount(async () => {
    messages = await invoke<TwitchMessage[]>("get_chat_messages");
    unlistenMessage = await listen('message', (event: {payload: TwitchMessage}) => {
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

    unlistenActiveUsers = await listen("active_users", (event: {payload: [totalUsers: number, activeUsers: number]}) => {
      activeUserStats = event.payload
    });

    activeUserStats = await invoke<[totalUsers: number, activeUsers: number]>("get_active_users");
    insultCount = await invoke<number>("get_insults_count");
    comebacksCount = await invoke<number>("get_comebacks_count");
  })

  onDestroy(() => {
    unlistenMessage();
    unlistenActiveUsers();
  })

  const scrollToBottom = async (node: Element) => node?.scroll({ top: node.scrollHeight, behavior: 'instant' })
</script>

<h1>Dashboard</h1>

<div class="md:flex justify-between my-4 gap-4">
  <a href='/insults' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Insults</p>
    <NumberTicker class="text-4xl font-bold mb-8" value={insultCount}></NumberTicker>
    <p class="text-muted-foreground">Insults loaded into the bot</p>
  </a>
  <a href='/comebacks' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Comebacks</p>
    <NumberTicker class="text-4xl font-bold mb-8" value={comebacksCount}></NumberTicker>
    <p class="text-muted-foreground">Reactions to users @-ing her</p>
  </a>
  <a href='/users' class="border rounded-xl p-6 hover:bg-muted">
    <p class="text-lg font-semibold">Users</p>
    <NumberTicker class="text-4xl font-bold mb-8" value={totalUsers}>
      <span class="text-muted-foreground text-sm">/ {activeUsers} Consented</span>
    </NumberTicker>
    <p class="text-muted-foreground">Users waiting to be insulted</p>
  </a>
</div>

<!-- <div class="flex justify-around my-4 gap-2">
</div> -->

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