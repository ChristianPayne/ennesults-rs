<script lang="ts">
  import { cubicOut } from "svelte/easing";
  import { onMount } from "svelte";
  import { tweened } from "svelte/motion";
  import { cn } from "$lib/utils";
  export let value = 100;
  export let initial = 0;
  export let duration = 750;
  let num = tweened(initial, {
    duration: duration,
    easing: cubicOut,
  });
  let className: any = "";
  export { className as class };

  $: num.set(value);
  
  onMount(() => {
    num.set(value);
  });
</script>

<div
  class={cn("inline-block tracking-normal", className)}
  {...$$restProps}
>
  {$num.toFixed(0)}
  <slot/>
</div>
