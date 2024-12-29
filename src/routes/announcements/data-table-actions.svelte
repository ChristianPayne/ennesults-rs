<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";

  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import Input from "$lib/components/ui/input/input.svelte";
  import type { Announcement } from "$lib/types";

  export let announcement: Announcement;

  // Sheet open controls.
  let showEditControls: boolean = false;

  // The text we are editing inside of the sheet.
  $: editText = announcement.value;

  function toggleEditControls() {
    showEditControls = !showEditControls;
  }

  function onOpenChange(value: boolean) {
    showEditControls = value;
  }

  async function deleteAnnouncement() {
    await invoke("delete_announcement", { announcementId: announcement.id });
  }

  async function updateAnnouncement() {
    if (editText == "") return;

    await invoke("update_announcement", {
      announcement: {
        id: announcement.id,
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
        Edit Announcement
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item on:click={deleteAnnouncement} class="text-destructive">
        Delete Announcement
      </DropdownMenu.Item>
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>

<Sheet.Root open={showEditControls} {onOpenChange}>
  <Sheet.Content side="right">
    <Sheet.Header>
      <Sheet.Title>Edit Announcement</Sheet.Title>
      <Sheet.Description>
        Make changes to the announcement here. Click save when you're done.
      </Sheet.Description>
    </Sheet.Header>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-left">Value</Label>
        <Input id="name" bind:value={editText} class="col-span-4" />
      </div>
    </div>
    <Sheet.Footer>
      <Button on:click={updateAnnouncement}>Save changes</Button>
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
