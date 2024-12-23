<script lang="ts">
  import type { Insult } from "$lib/types";
  import { derived, get, writable, type Writable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import DataTableActions from "./data-table-actions.svelte";
  import EditInsult from "$lib/components/editInsult.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import EditInsultTags from "$lib/components/editInsultTags.svelte";

  export let insults: Writable<Insult[]>;

  const table = createTable(insults);

  let insultBeingEdited: Writable<string> = writable("");

  function setInsultBeingEdited(id: string) {
    insultBeingEdited.set(id);
  }

  async function updateInsult(
    insult: Insult | undefined,
    closeAfterSave: boolean = true,
  ) {
    if (!get(insultBeingEdited)) return;
    if (insult === undefined) {
      insultBeingEdited.set("");
      return;
    }
    if (insult.value == "") return;

    await invoke("update_insult", {
      insult,
    }).then(() => {
      console.log("Saved insult", insult);
    });
    if (closeAfterSave) {
      insultBeingEdited.set("");
    }
  }

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: (insult) => insult,
      header: "Insult",
      cell: ({ value }) =>
        createRender(
          EditInsult,
          derived(insultBeingEdited, (insultBeingEdited) => ({
            insultBeingEdited,
            insult: value,
            callback: updateInsult,
          })),
        ),
    }),
    table.column({
      accessor: (insult) => insult,
      header: "Tags",
      cell: ({ value }) => {
        return createRender(
          EditInsultTags,
          derived(insultBeingEdited, (insultBeingEdited) => ({
            insultBeingEdited,
            insult: value,
            callback: updateInsult,
          })),
        );
      },
    }),
    table.column({
      accessor: "id",
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, {
          id: value,
          setInsultBeingEdited,
        });
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
    table.createViewModel(columns);
</script>

<div class="rounded-md">
  <Table.Root {...$tableAttrs}>
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                <Table.Head {...attrs}>
                  <Render of={cell.render()} />
                </Table.Head>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Header>
    <Table.Body {...$tableBodyAttrs} class="select-auto">
      {#each $pageRows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row {...rowAttrs}>
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell {...attrs}>
                  <Render of={cell.render()} />
                </Table.Cell>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
