<script lang="ts">
  import type { Insult } from "$lib/types";
  import { type Writable } from "svelte/store";
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

  export let insults: Writable<Insult[]>;

  const table = createTable(insults);

  let insultBeingEdited: string | null = null;

  function setInsultBeingEdited(id: string) {
    insultBeingEdited = id;
  }

  async function updateInsult(insultValue: string) {
    if (!insultBeingEdited) return;
    if (insultValue == "") {
      insultBeingEdited = null;
      return;
    }
    await invoke("update_insult", {
      insult: { id: insultBeingEdited, value: insultValue },
    });
    insultBeingEdited = null;
  }

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: (x) => x,
      header: "Insult",
      cell: ({ value: insult }) => {
        if (insult.id === insultBeingEdited) {
          return createRender(EditInsult, {
            insultValue: insult.value,
            callback: updateInsult,
          });
        }

        return insult.value;
      },
    }),
    table.column({
      accessor: ({ id }) => id,
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, {
          id: value,
          editInsultCallback: setInsultBeingEdited,
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
                  {#if cell.value.id === insultBeingEdited}
                    <EditInsult
                      insultValue={cell.value.value}
                      callback={updateInsult}
                    />
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Cell>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Body>
  </Table.Root>
</div>
