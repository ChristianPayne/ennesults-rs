<script lang="ts">
  import "../styles.css";
  import { Badge, DarkMode, Button, Popover } from 'flowbite-svelte';

  let connectionStatus = true;
  let channelName = "Ennegineer";
</script>

<div class="flex flex-col h-full">
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
  <!-- Footer -->
  <div class="bg-primary-600 py-1 px-2 flex">
    <div class="grow"></div>
    <!-- Badges -->
    <div class="flex space-x-2">
      <div>
        <Badge id="connectionStatus" color="{connectionStatus ? 'green' : 'red'}">
          {connectionStatus ? channelName : "Disconnected"}
        </Badge>
        <Popover title="Channel" triggeredBy="#connectionStatus" arrow={false}>
          { connectionStatus ? `We are connected to ${channelName}.` : "Not connected to a channel." }
        </Popover>
      </div>
    </div>
  </div>
</div>
