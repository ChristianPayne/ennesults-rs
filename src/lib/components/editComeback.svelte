<script lang="ts">
  import type { Comeback } from "$lib/types";
  import Input from "./ui/input/input.svelte";

  export let callback: (comebackValue: string) => void;
  export let comebackBeingEdited: string;
  export let comeback: Comeback;

  let value = comeback.value;

  function onKeyDown(e) {
    switch (e.keyCode) {
      case 13: {
        // Enter
        callback(comeback.value);
        break;
      }
      case 27: {
        // Escape
        value = comeback.value;
        callback("");
        break;
      }
    }
  }
</script>

{#if comebackBeingEdited === comeback.id}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:keydown={onKeyDown}>
    <Input bind:value></Input>
  </div>
{:else}
  <p>{comeback.value}</p>
{/if}
