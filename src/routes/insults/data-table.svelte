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

  export let insults: Writable<Insult[]>;

  const table = createTable(insults);

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: "value",
      header: "Insult",
    }),
    table.column({
      accessor: ({ id }) => id,
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, { id: value });
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
