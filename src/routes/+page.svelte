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