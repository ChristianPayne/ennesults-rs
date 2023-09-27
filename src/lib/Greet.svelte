<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>

<div>
  <form class="text-white" on:submit|preventDefault={greet}>
    <input class="text-gray-500 p-2 rounded-md" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
</div>