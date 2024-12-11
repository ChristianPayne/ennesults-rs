<script lang="ts">
  import Input from "./ui/input/input.svelte";

  export let callback: (insultValue: string) => void;
  export let insultValue = "";

  function onKeyDown(e) {
    switch (e.keyCode) {
      case 13: {
        // Enter
        callback(insultValue);
        break;
      }
      case 27: {
        // Escape
        insultValue = "";
        callback(insultValue);
        break;
      }
    }
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:keydown={onKeyDown}>
  <Input bind:value={insultValue} focusOnMount></Input>
</div>
