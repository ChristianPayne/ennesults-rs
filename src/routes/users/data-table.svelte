<script lang="ts">
  import { createTable, Render, Subscribe, createRender } from "svelte-headless-table";
  import { readable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import DataTableActions from "./data-table-actions.svelte";
    import Badge from "$lib/components/ui/badge/badge.svelte";
    import { Checkbox } from "$lib/components/ui/checkbox";




  type EnnesultsTableItem = {
    id: number,
    user: string,
    type: string,
    consented: boolean
  }

  let data: EnnesultsTableItem[] = [
    { id: 1, user: 'ChrisGriffin522', type: 'Moderator', consented: true },
    { id: 2, user: 'Ennegineer', type: 'Streamer', consented: true },
    { id: 3, user: 'JinJix', type: 'Viewer', consented: false },
    { id: 4, user: 'jaypez04', type: 'Viewer', consented: false },
    { id: 5, user: 'mcgyver0302', type: 'Viewer', consented: true },
  ];

  const table = createTable(readable(data));

  const columns = table.createColumns([
    table.column({
      accessor: "id",
      header: "ID",
    }),
    table.column({
      accessor: "user",
      header: "User",
    }),
    table.column({
      accessor: "type",
      header: "Type",
    }),
    table.column({
      accessor: "consented",
      header: "Consented",
      cell: ({value}) => {
        return createRender(Checkbox, {checked: value})
      }
    }),
    table.column({
      accessor: ({ id }) => id,
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, { id: value });
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
    <Table.Body {...$tableBodyAttrs}>
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