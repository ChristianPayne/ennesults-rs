<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { writable, get } from "svelte/store";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import type { Comeback } from "$lib/types";
  import DataTable from "./data-table.svelte";
  import { customAlphabet } from "nanoid";
  const nanoid = customAlphabet('0123456789abcdefghijklmnopqrstuvwxyz', 8)

  let comebacks$ = writable<Comeback[]>([]);

  let input: string;

  let unlisten: UnlistenFn;

  onMount(async () => {
    let comebacks = await invoke<Comeback[]>("get_comebacks");
    comebacks$.set(comebacks);

    unlisten = await listen<Comeback[]>("comebacks_update", event => {
      comebacks$.set(event.payload);
    })
  })

  onDestroy(() => {
    unlisten?.();
  })

  async function saveComeback() {
    if(input.trim() === "") {
      return
    }

    let currentComebacks = get(comebacks$);
    let newComeback: Comeback = {
      id: nanoid(),
      value: input.trim()
    }

    await invoke("save_comebacks", { comebacks: [newComeback, ...currentComebacks] })

    input = "";
  }
</script>

<div class="flex flex-col gap-4">
  <h1>Comebacks</h1>
  
  <form on:submit={() => saveComeback()} class="flex gap-2">
    <Input type="text" bind:value={input} />
    <Button type="submit">Add</Button>
  </form>
  
  {#if comebacks$}
  <DataTable comebacks={comebacks$} />
  {/if}
</div>