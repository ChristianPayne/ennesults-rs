<script lang="ts">
  import type { Insult, InsultTag } from "$lib/types";
  import Input from "./ui/input/input.svelte";
  import { Badge } from "$lib/components/ui/badge";
  import * as Select from "$lib/components/ui/select/index.js";
  import type { Selected } from "bits-ui";

  export let callback: (
    insult: Insult | undefined,
    closeAfterSave?: boolean,
  ) => void;
  export let insultBeingEdited: string;
  export let insult: Insult;

  const insultTagOptions: InsultTag[] = [
    "Insult",
    "Consent",
    "Unconsent",
    "Raid",
    "Lurk",
  ];

  function onTagsChange(value: Selected<InsultTag>[]) {
    const newTags = value.map((x) => x.value);
    insult.tags = newTags;
  }

  function saveTags() {
    callback(insult, false);
  }
</script>

{#if insultBeingEdited === insult.id}
  <Select.Root
    portal={null}
    multiple={true}
    onSelectedChange={onTagsChange}
    onOutsideClick={saveTags}
    selected={insult.tags.map((t) => ({ label: t, value: t }))}
  >
    <Select.Trigger class="w-[180px]">
      <Select.Value placeholder="Select tags..." />
    </Select.Trigger>
    <Select.Content>
      <Select.Group>
        <Select.Label>Tags</Select.Label>
        {#each insultTagOptions as tag}
          <Select.Item value={tag} label={tag}>{tag}</Select.Item>
        {/each}
      </Select.Group>
    </Select.Content>
    <Select.Input name="favoriteFruit" />
  </Select.Root>
{:else}
  <div class="flex space-x-2">
    {#each insult.tags as tag}
      <Badge variant="outline">{tag}</Badge>
    {/each}
  </div>
{/if}
