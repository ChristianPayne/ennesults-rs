<script lang="ts">
  import type { Insult } from "$lib/types";
  import Input from "./ui/input/input.svelte";

  export let callback: (
    insult: Insult | undefined,
    closeAfterSave?: boolean,
  ) => void;
  export let insultBeingEdited: string;
  export let insult: Insult;

  let value = insult.value;

  function onKeyDown(e) {
    switch (e.keyCode) {
      case 13: {
        // Enter
        insult.value = value;
        callback(insult);
        break;
      }
      case 27: {
        // Escape
        value = insult.value;
        callback(undefined);
        break;
      }
    }
  }
</script>

{#if insultBeingEdited === insult.id}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:keydown={onKeyDown}>
    <Input bind:value></Input>
  </div>
{:else}
  <p>{insult.value}</p>
{/if}
