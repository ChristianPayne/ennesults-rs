<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import { invoke } from "@tauri-apps/api/tauri"
  
  async function connect () {
    let status = await invoke("connect_to_channel");
    console.log('🛠 Status', status);
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

  <button on:click={connect}>
    Connect to Ennegineer!
  </button>

  <Greet />
</div>