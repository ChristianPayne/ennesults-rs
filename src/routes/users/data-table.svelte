<script lang="ts">
  import { createTable, Render, Subscribe, createRender } from "svelte-headless-table";
  import { type Writable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import DataTableActions from "./data-table-actions.svelte";
  import Badge from "$lib/components/ui/badge/badge.svelte";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import type { User } from "$lib/types";

  export let usersStore: Writable<User[]>;

  const table = createTable(usersStore);

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: "username",
      header: "Username",
    }),
    table.column({
      accessor: "last_seen",
      header: "Last Seen",
    }),
    table.column({
      accessor: "consented",
      header: "Consented",
      cell: ({value}) => {
        return createRender(Checkbox, { checked: value, disabled: true })
      }
    }),
    table.column({
      accessor: (row) => row,
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, { id: value.id, username: value.username });
      },
    }),
  ]);


  const { headerRows, pageRows, tableAttrs, tableBodyAttrs } = table.createViewModel(columns);

</script>


<div class="">
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
    <Table.Body {...$tableBodyAttrs} class="select-text selection:text-primary-foreground">
      {#each $pageRows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row {...rowAttrs}>
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell {...attrs}>
                  {#if cell.id === 'type'}
                    <Badge>{cell.render()}</Badge>
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