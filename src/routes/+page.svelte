<script lang="ts">
  import Greet from '$lib/Greet.svelte'
  import { invoke } from "@tauri-apps/api/tauri"
  import { Button } from 'flowbite-svelte';
  
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
</script>

<h1 class="text-4xl h-24 text-center flex items-center 
text-primary-900
bg-gradient-to-b from-primary-500 to-primary-100
hover:bg-primary-900 p-4 rounded-2xl m-8">
  <p class="w-full">Welcome, Ennegineer!</p>
</h1>

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
</div>  

<Greet />