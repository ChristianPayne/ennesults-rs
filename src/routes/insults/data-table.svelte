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
  import InsultTags from "$lib/components/insultTags.svelte";

  export let insults: Writable<Insult[]>;
  const table = createTable(insults);

  const columns = table.createColumns([
    table.column({
      accessor: "value",
      header: "Insult",
    }),
    table.column({
      accessor: (insult) => insult,
      header: "Tags",
      cell: ({ value: insult }) => {
        return createRender(InsultTags, {
          insult,
        });
      },
    }),
    table.column({
      accessor: (insult) => insult,
      header: "Actions",
      cell: ({ value: insult }) => {
        return createRender(DataTableActions, {
          insult,
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
