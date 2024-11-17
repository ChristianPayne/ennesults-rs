<script lang="ts">
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { writable, get } from "svelte/store";
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
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
  
  <div class="flex gap-2 w-full">
    <form on:submit={() => saveComeback()} class="flex gap-2 w-full">
      <Input type="text" bind:value={input} />
      <Button type="submit">Add</Button>
    </form>
    <AlertDialog.Root>
      <AlertDialog.Trigger>
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
          <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z" />
        </svg>  
      </AlertDialog.Trigger>
      <AlertDialog.Content>
        <AlertDialog.Header>
          <AlertDialog.Title>Insult Substitutions</AlertDialog.Title>
          <AlertDialog.Description>
            Insults can have dynamic data inside of them. Use the &lbrace;&lbrace;handlebars&rbrace;&rbrace; syntax to swap out the dynamic data.
          </AlertDialog.Description>
          <AlertDialog.Description>
            <span class="font-semibold">&lbrace;&lbrace;streamer&rbrace;&rbrace;</span>: Replaced with the channel name.
          </AlertDialog.Description>
          <AlertDialog.Description>
            <span class="font-semibold">&lbrace;&lbrace;user&rbrace;&rbrace;</span>: Replaced with a random user that is active and has consented.
          </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
          <AlertDialog.Cancel>Got it</AlertDialog.Cancel>
        </AlertDialog.Footer>
      </AlertDialog.Content>
    </AlertDialog.Root>
  </div>
  
  {#if comebacks$}
  <DataTable comebacks={comebacks$} />
  {/if}
</div>