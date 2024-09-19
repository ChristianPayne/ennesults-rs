<script lang="ts">
  import { invoke } from "@tauri-apps/api/core"

  let message = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("say", { message })
  }
</script>

<div>
  <form class="text-white" on:submit|preventDefault={greet}>
    <input class="text-gray-500 p-2 rounded-md" placeholder="Send a message..." bind:value={message} />
    <button type="submit">Send</button>
  </form>
  <p>{greetMsg}</p>
</div>