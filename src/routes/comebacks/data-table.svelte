<script lang="ts">
  import type { Comeback } from "$lib/types";
  import { writable, get, type Writable, derived } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import DataTableActions from "./data-table-actions.svelte";
  import EditComeback from "$lib/components/editComeback.svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let comebacks: Writable<Comeback[]>;

  const table = createTable(comebacks);

  let comebackBeingEdited: Writable<string> = writable("");

  async function updateComeback(comebackValue: string) {
    if (!get(comebackBeingEdited)) return;
    if (comebackValue == "") {
      comebackBeingEdited.set("");
      return;
    }
    await invoke("update_comeback", {
      comeback: { id: get(comebackBeingEdited), value: comebackValue },
    });
    comebackBeingEdited.set("");
  }

  function setComebackBeingEdited(id: string) {
    comebackBeingEdited.set(id);
  }

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: (comeback) => comeback,
      header: "Comeback",
      cell: ({ value }) =>
        createRender(
          EditComeback,
          derived(comebackBeingEdited, (comebackBeingEdited) => ({
            comebackBeingEdited,
            comeback: value,
            callback: updateComeback,
          })),
        ),
    }),
    table.column({
      accessor: "id",
      header: "Actions",
      cell: ({ value }) =>
        createRender(DataTableActions, {
          id: value,
          setComebackBeingEdited,
        }),
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
