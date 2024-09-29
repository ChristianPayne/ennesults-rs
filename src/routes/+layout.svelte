<script lang="ts">
  import "../styles.css";
  import { Badge, DarkMode, Button, Popover } from 'flowbite-svelte';
  import { emit, listen } from '@tauri-apps/api/event'

  let connectionStatus = false;
  let channelName = "none";

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
</script>

<div class="flex flex-col h-full">
  <!-- Sidebar -->
  <!-- <div class="bg-primary-600 dark:bg-primary-900 
  w-[250px] min-w-[215px] max-w-[215px] p-2">
    Lorem ipsum dolor sit amet consectetur adipisicing elit. Libero in quis labore perferendis possimus assumenda perspiciatis provident magnam quaerat. Architecto vero iusto dolore deleniti asperiores earum quas culpa delectus odio.
  </div> -->
  <div class="flex flex-col grow">
    <!-- Title -->
    <div class="flex justify-between m-4">
      <a href="/">
        <h1 class="font-sans">
          Ennesults
        </h1>
      </a>
      <div class="flex space-x-2 items-center">
        <a href="/chat">Chat</a>
        <a href="/commands">Commands</a>
        <a href="/ennesults">Ennesults</a>
        <a href="/users">Users</a>
        <a href="/settings">Settings</a>
        <DarkMode size="md"/>
      </div>
    </div>
    <!-- Main Content -->
    <div class="grow mx-4 overflow-y-scroll">
      <slot/>
    </div>
  </div>
  <!-- Footer -->
  <div class="bg-primary-600 py-1 px-2 flex border-t border-primary-800 rounded-t-md">
    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2">
      <div>
        <Badge id="connectionStatus" color="{connectionStatus ? 'green' : 'red'}">
          {connectionStatus ? channelName : "Disconnected"}
        </Badge>
        <Popover title="Channel" color="primary" triggeredBy="#connectionStatus" arrow={false}>
          { connectionStatus ? `We are connected to ${channelName}.` : "Not connected to a channel." }
        </Popover>
      </div>
    </div>
  </div>
</div>
