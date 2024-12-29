<script lang="ts">
  import { customAlphabet } from "nanoid";
  const nanoid = customAlphabet("0123456789abcdefghijklmnopqrstuvwxyz", 8);
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { writable, get } from "svelte/store";
  import DataTable from "./data-table.svelte";
  import type { Announcement } from "$lib/types";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Input } from "$lib/components/ui/input/index.js";

  let announcements$ = writable<Announcement[]>([]);

  let input: string;

  let unlisten: UnlistenFn;

  onMount(async () => {
    let announcements = await invoke<Announcement[]>("get_announcements");
    announcements$.set(announcements);

    unlisten = await listen<Announcement[]>("announcements_update", (event) => {
      announcements$.set(event.payload);
    });
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function saveNewAnnouncement() {
    if (input.trim() === "") {
      return;
    }

    let currentAnnouncements = get(announcements$);
    let newAnnouncement: Announcement = {
      id: nanoid(),
      value: input.trim(),
    };

    await invoke("save_announcements", {
      announcements: [newAnnouncement, ...currentAnnouncements],
    });

    input = "";
  }
</script>

<div class="flex flex-col gap-4">
  <h1>Announcements</h1>

  <div class="flex gap-2 w-full">
    <form on:submit={() => saveNewAnnouncement()} class="flex gap-2 w-full">
      <Input type="text" bind:value={input} placeholder="Add announcement..." />
      <Button type="submit">Add</Button>
    </form>
  </div>

  {#if announcements$}
    <DataTable announcements={announcements$} />
  {/if}
</div>
