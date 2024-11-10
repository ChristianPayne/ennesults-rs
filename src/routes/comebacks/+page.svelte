<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import type { Comeback } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DataTable from "./data-table.svelte";
  
  let comebacks: Comeback[];

  onMount(async () => {
    comebacks = await invoke("get_comebacks");
  })

  async function saveComeback() {
    console.log("saveComeback")
    await invoke("save_comebacks", { comebacks: [
        {
          id: 0,
          value: "Here is a comeback"
        }
      ] 
    })
  }
</script>

<h1>Comebacks</h1>

<input type="text">
<Button on:click={() => saveComeback()}>SAVE</Button>

{#if comebacks}
<DataTable comebacks={comebacks} />
{/if}