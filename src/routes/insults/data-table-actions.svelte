<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  import type { Insult, InsultTag } from "$lib/types";
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import Input from "$lib/components/ui/input/input.svelte";
  import * as Select from "$lib/components/ui/select/index.js";
  import type { Selected } from "bits-ui";

  export let insult: Insult;

  const insultTagOptions: InsultTag[] = [
    "Insult",
    "Consent",
    "Unconsent",
    "Raid",
    "Lurk",
  ];

  // Sheet open controls.
  let showEditControls: boolean = false;

  // The text we are editing inside of the sheet.
  $: editText = insult.value;

  function toggleEditControls() {
    showEditControls = !showEditControls;
  }

  function onOpenChange(value: boolean) {
    showEditControls = value;
  }

  async function deleteInsult() {
    await invoke("delete_insult", {
      insultId: insult.id,
    });
  }

  async function updateInsult() {
    if (editText == "") return;

    await invoke("update_insult", {
      insult: {
        id: insult.id,
        value: editText,
        tags: insult.tags,
      },
    });

    showEditControls = false;
  }

  function onTagsChange(value: Selected<InsultTag>[]) {
    const newTags = value.map((x) => x.value);
    insult.tags = newTags;
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
        Edit Insult
      </DropdownMenu.Item>
      <DropdownMenu.Separator />
      <DropdownMenu.Item on:click={deleteInsult} class="text-destructive">
        Delete Insult
      </DropdownMenu.Item>
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>

<Sheet.Root open={showEditControls} {onOpenChange}>
  <Sheet.Content side="right">
    <Sheet.Header>
      <Sheet.Title>Edit Insult</Sheet.Title>
      <Sheet.Description>
        Make changes to the insult here. Click save when you're done.
      </Sheet.Description>
    </Sheet.Header>
    <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-left">Value</Label>
        <Input id="name" bind:value={editText} class="col-span-4" />
      </div>
      <div class="grid gap-4 py-4">
        <Label for="tags" class="text-left">Tags</Label>
        <Select.Root
          portal={null}
          multiple={true}
          onSelectedChange={onTagsChange}
          selected={insult.tags.map((t) => ({ label: t, value: t }))}
        >
          <Select.Trigger class="col-span-4">
            <Select.Value placeholder="Select tags..." />
          </Select.Trigger>
          <Select.Content>
            <Select.Group>
              {#each insultTagOptions as tag}
                <Select.Item value={tag} label={tag}>{tag}</Select.Item>
              {/each}
            </Select.Group>
          </Select.Content>
        </Select.Root>
      </div>
    </div>
    <Sheet.Footer>
      <Button on:click={updateInsult}>Save changes</Button>
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>
