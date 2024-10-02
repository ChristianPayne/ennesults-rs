<script lang="ts">
	import { Checkbox as CheckboxPrimitive } from "bits-ui";
	import { cn } from "$lib/utils.js";

	type $$Props = CheckboxPrimitive.Props;
	type $$Events = CheckboxPrimitive.Events;

	let className: $$Props["class"] = undefined;
	export let checked: $$Props["checked"] = false;
	export { className as class };
</script>

<CheckboxPrimitive.Root
	class={cn(
		"border-primary focus-visible:ring-ring data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground peer box-content h-4 w-4 shrink-0 rounded-sm border shadow focus-visible:outline-none focus-visible:ring-1 disabled:cursor-not-allowed disabled:opacity-50 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50",
		className
	)}
	bind:checked
	on:click
	{...$$restProps}
>
	<CheckboxPrimitive.Indicator
		class={cn("flex h-4 w-4 items-center justify-center text-current")}
		let:isChecked
		let:isIndeterminate
	>
		{#if isIndeterminate}
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
				<path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14" />
			</svg>			
		{:else}
			<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class={cn("h-3.5 w-3.5", !isChecked && "opacity-0")} >
				<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
			</svg>			
		{/if}
	</CheckboxPrimitive.Indicator>
</CheckboxPrimitive.Root>
