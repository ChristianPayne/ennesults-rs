<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import { invoke } from "@tauri-apps/api/tauri"
  
  async function connect_to_channel () {
    let status = await invoke("connect_to_channel");
    console.log('🛠 Status', status);
  }
  async function print_state () {
    let state = await invoke("print_state");
    console.log('🛠 print_state', state);
  }
</script>

<div class="flex-col text-center space-y-5">
  <!-- Title -->
  <div>
    <h1>
      Ennesults
    </h1>
    <h2>
      Rust Edition
    </h2>
  </div>

  <button on:click={connect_to_channel}>
    Connect to Ennegineer!
  </button>
  <button on:click={print_state}>
    Print State!
  </button>

  <Greet />
</div>