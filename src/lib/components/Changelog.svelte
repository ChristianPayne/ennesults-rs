<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import Button from "./ui/button/button.svelte";

  export let changelog: { version: string; notes: string }[];

  function formatChangelog(changelog) {
    return changelog?.map((log) => {
      return {
        version: log.version,
        title: log.title,
        notes: log.notes.replaceAll("\n", "<br/>"),
      };
    });
  }
</script>

<Dialog.Root>
  <Dialog.Header>
    <Dialog.Title>Changelog</Dialog.Title>
  </Dialog.Header>
  <Dialog.Description
    class="max-h-96 overflow-y-scroll flex flex-col gap-4 text-left"
  >
    {#each formatChangelog(changelog) as log}
      <div>
        <h2 class="font-bold">
          {#if log.title}
            {log.title} -
          {/if}
          <span class="font-normal">
            {log.version}
          </span>
        </h2>
        {@html log.notes}
      </div>
    {/each}
  </Dialog.Description>
</Dialog.Root>
