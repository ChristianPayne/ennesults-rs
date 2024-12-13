<script lang="ts">
  import type { Announcement } from "$lib/types";
  import Input from "./ui/input/input.svelte";

  export let callback: (announcementValue: string) => void;
  export let announcementBeingEdited: string;
  export let announcement: Announcement;

  let value = announcement.value;

  function onKeyDown(e) {
    switch (e.keyCode) {
      case 13: {
        // Enter
        callback(value);
        announcement.value = value;
        break;
      }
      case 27: {
        // Escape
        value = announcement.value;
        callback("");
        break;
      }
    }
  }
</script>

{#if announcementBeingEdited === announcement.id}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:keydown={onKeyDown}>
    <Input bind:value></Input>
  </div>
{:else}
  <p>{announcement.value}</p>
{/if}
