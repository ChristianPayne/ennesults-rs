<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import Input from "$lib/components/ui/input/input.svelte";
  import type { Comeback } from "$lib/types";

  export let comeback: Comeback;

  // Sheet open controls.
  let showEditControls: boolean = false;

  // The text we are editing inside of the sheet.
  let editText: string = "";

  function toggleEditControls() {
    showEditControls = !showEditControls;
    if (showEditControls === true) {
      editText = comeback.value;
    }
  }

  function onOpenChange(value: boolean) {
    showEditControls = value;
  }

  async function deleteComeback() {
    await invoke("delete_comeback", { comebackId: comeback.id });
  }

  async function updateComeback() {
    if (editText == "") return;

    await invoke("update_comeback", {
      comeback: {
        id: comeback.id,
        value: editText,
      },
    });

    showEditControls = false;
  }
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button
      variant="ghost"
      builders={[builder]}
      size="icon"
      class="relative h-8 w-8 p-0"
    >
      <span class="sr-only">Open menu</span>
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
          d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z"
        />
      </svg>
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
    <DropdownMenu.Group>
      <DropdownMenu.Label>Actions</DropdownMenu.Label>
      <DropdownMenu.Item on:click={toggleEditControls}>
        Edit Comeback
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item on:click={deleteComeback} class="text-destructive">
        Delete comeback
      </DropdownMenu.Item>
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>

<Sheet.Root open={showEditControls} {onOpenChange}>
  <Sheet.Content side="right">
    <Sheet.Header>
      <Sheet.Title>Edit Comeback</Sheet.Title>
      <Sheet.Description>
        Make changes to the comeback here. Click save when you're done.
      </Sheet.Description>
    </Sheet.Header>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-left">Value</Label>
        <Input id="name" bind:value={editText} class="col-span-4" />
      </div>
    </div>
    <Sheet.Footer>
      <Button on:click={updateComeback}>Save changes</Button>
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
