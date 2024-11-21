<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { type TwitchMessage } from "$lib/types";
  import NumberTicker from "$lib/components/NumberTicker.svelte";
  import { goto } from "$app/navigation";

  const maxChatMessages = 100;

  let messages: TwitchMessage[] = [];
  let unlistenMessage: UnlistenFn;
  let unlistenActiveUsers: UnlistenFn;

  let chatElement: Element;

  let activeUserStats: [totalUsers: number, activeUsers: number] = [0, 0];

  let insultCount = 0;
  let comebacksCount = 0;

  $: totalUsers = activeUserStats[0];
  $: activeUsers = activeUserStats[1];

  $: messages, scrollToBottom(chatElement);

  onMount(async () => {
    messages = await invoke<TwitchMessage[]>("get_chat_messages");
    unlistenMessage = await listen(
      "message",
      (event: { payload: TwitchMessage }) => {
        // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
        // event.payload is the payload object
        // console.log("event", event.payload)
        messages.push({
          username: event.payload.username,
          message: event.payload.message,
          color: event.payload.color,
        });
        messages = messages.slice(-maxChatMessages);

        return event;
      },
    );

    unlistenActiveUsers = await listen(
      "active_users",
      (event: { payload: [totalUsers: number, activeUsers: number] }) => {
        activeUserStats = event.payload;
      },
    );

    activeUserStats =
      await invoke<[totalUsers: number, activeUsers: number]>(
        "get_active_users",
      );
    insultCount = await invoke<number>("get_insults_count");
    comebacksCount = await invoke<number>("get_comebacks_count");
  });

  onDestroy(() => {
    unlistenMessage();
    unlistenActiveUsers();
  });

  const scrollToBottom = async (node: Element) =>
    node?.scroll({ top: node.scrollHeight, behavior: "instant" });
</script>

<h1>Dashboard</h1>

<div
  class="grid grid-cols-1 sm:grid-cols-3 gap-2 my-4 sm:gap-4 children:border children:rounded-xl children:p-4 children:sm:p-6 hover:children:bg-muted children:w-full children:text-left children:items-end"
>
  <button on:click={() => goto("/insults")}>
    <p class="text-lg font-semibold">Insults</p>
    <NumberTicker class="text-4xl font-bold" value={insultCount}></NumberTicker>
    <p class="text-muted-foreground">Insults loaded into the bot</p>
  </button>
  <button on:click={() => goto("/comebacks")}>
    <p class="text-lg font-semibold">Comebacks</p>
    <NumberTicker class="text-4xl font-bold" value={comebacksCount}
    ></NumberTicker>
    <p class="text-muted-foreground">Reactions to users @-ing her</p>
  </button>
  <button on:click={() => goto("/users")}>
    <p class="text-lg font-semibold">Users</p>
    <NumberTicker class="text-4xl font-bold" value={totalUsers}>
      <span class="text-muted-foreground text-sm"
        >/ {activeUsers} Consented</span
      >
    </NumberTicker>
    <p class="text-muted-foreground">Users waiting to be insulted</p>
  </button>
</div>

<!-- <div class="flex justify-around my-4 gap-2">
</div> -->

<div class="flex space-x-4">
  <h1>Chat</h1>
  <Button variant="ghost" on:click={() => (messages = [])}>Clear Chat</Button>
</div>
<div
  class="overflow-y-scroll max-h-screen min-h-[65vh] h-auto select-text border rounded-md p-2"
  bind:this={chatElement}
>
  <ul class="space-y-1">
    {#each messages as message}
      <li>
        <span
          style="color: rgb({message.color?.[0]},{message.color?.[1]},{message
            .color?.[2]});"
          class="text-primary">{message.username}</span
        >: {message.message}
      </li>
    {/each}
  </ul>
</div>
