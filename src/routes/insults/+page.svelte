<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { writable, get } from "svelte/store";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import type { Insult } from "$lib/types";
  import { customAlphabet } from "nanoid";
  const nanoid = customAlphabet('0123456789abcdefghijklmnopqrstuvwxyz', 8)
  import DataTable from "./data-table.svelte";

  let insults$ = writable<Insult[]>([]);

  let input: string;

  let unlisten: UnlistenFn;

  onMount(async () => {
    let insults = await invoke<Insult[]>("get_insults");
    insults$.set(insults);

    unlisten = await listen<Insult[]>("insults_update", event => {
      insults$.set(event.payload);
    })
  })

  onDestroy(() => {
    unlisten?.();
  })

  async function saveInsult() {
    if(input.trim() === "") {
      return
    }

    let currentInsults = get(insults$);
    let newInsult: Insult = {
      id: nanoid(),
      value: input.trim()
    }

    await invoke("save_insults", { insults: [newInsult, ...currentInsults] })

    input = "";
  }
</script>

<div class="flex flex-col gap-4">
  <h1>Insults</h1>
  
  <form on:submit={() => saveInsult()} class="flex gap-2">
    <Input type="text" bind:value={input} />
    <Button type="submit">Add</Button>
  </form>
  
  {#if insults$}
  <DataTable insults={insults$} />
  {/if}
</div>