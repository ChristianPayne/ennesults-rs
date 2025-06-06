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
  import * as AlertDialog from "$lib/components/ui/alert-dialog";

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
    <AlertDialog.Root>
      <AlertDialog.Trigger>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="size-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
          />
        </svg>
      </AlertDialog.Trigger>
      <AlertDialog.Content>
        <AlertDialog.Header>
          <AlertDialog.Title>Substitutions</AlertDialog.Title>
          <AlertDialog.Description>
            Ennesults can use dynamic data. Use the
            &lbrace;&lbrace;handlebars&rbrace;&rbrace; syntax to swap out the
            dynamic data.
          </AlertDialog.Description>
          <AlertDialog.Description>
            <span class="font-semibold select-text"
              >&lbrace;&lbrace;streamer&rbrace;&rbrace;</span
            >: Replaced with the channel name.
          </AlertDialog.Description>
          <AlertDialog.Description>
            <span class="font-semibold select-text"
              >&lbrace;&lbrace;user&rbrace;&rbrace;</span
            >: Replaced with a random user that is active and has consented.
            This can be used multiple times and will be the same user each time.
          </AlertDialog.Description>
          <AlertDialog.Description>
            <span class="font-semibold select-text"
              >&lbrace;&lbrace;version&rbrace;&rbrace;</span
            >: Replaced with the current version number of the bot.
          </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
          <AlertDialog.Cancel>Got it</AlertDialog.Cancel>
        </AlertDialog.Footer>
      </AlertDialog.Content>
    </AlertDialog.Root>
  </div>

  {#if announcements$}
    <DataTable announcements={announcements$} />
  {/if}
</div>
