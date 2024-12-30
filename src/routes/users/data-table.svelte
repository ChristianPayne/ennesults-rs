<script lang="ts">
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import { type Writable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import DataTableActions from "./data-table-actions.svelte";
  import Badge from "$lib/components/ui/badge/badge.svelte";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { Button } from "$lib/components/ui/button";
  import type { User } from "$lib/types";
  import {
    addTableFilter,
    addPagination,
    addSortBy,
  } from "svelte-headless-table/plugins";
  import { Input } from "$lib/components/ui/input";

  export let usersStore: Writable<User[]>;

  const table = createTable(usersStore, {
    page: addPagination({
      initialPageSize: 50,
    }),
    sort: addSortBy(),
    filter: addTableFilter({
      fn: ({ filterValue, value }) =>
        value.toLowerCase().includes(filterValue.toLowerCase()),
    }),
  });

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
      plugins: {
        filter: {
          exclude: true,
        },
      },
    }),
    table.column({
      accessor: "consented",
      header: "Consented",
      plugins: {
        sort: {
          compareFn: (left, right) => {
            if (left && !right) return -1;
            if (!left && right) return 1;
            return 0;
          },
        },
        filter: {
          exclude: true,
        },
      },
      cell: ({ value }) => {
        return createRender(Checkbox, {
          checked: value,
          disabled: true,
          class: "!opacity-100",
        });
      },
    }),
    table.column({
      accessor: (row) => row,
      header: "Actions",
      plugins: {
        filter: {
          exclude: true,
        },
        sort: {
          disable: true,
        },
      },
      cell: ({ value }) => {
        return createRender(DataTableActions, {
          // id: value.id,
          username: value.username,
        });
      },
    }),
  ]);

  const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates } =
    table.createViewModel(columns);

  const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
  const { filterValue } = pluginStates.filter;
</script>

<div class="flex flex-col">
  <div class="flex items-center py-4">
    <Input
      class=""
      placeholder="Find username..."
      type="text"
      bind:value={$filterValue}
    />
  </div>
  <Table.Root {...$tableAttrs} class="">
    <Table.Header>
      {#each $headerRows as headerRow}
        <Subscribe rowAttrs={headerRow.attrs()}>
          <Table.Row>
            {#each headerRow.cells as cell (cell.id)}
              <Subscribe
                attrs={cell.attrs()}
                let:attrs
                props={cell.props()}
                let:props
              >
                <Table.Head {...attrs}>
                  {#if cell.id === "username" || cell.id === "last_seen" || cell.id == "consented"}
                    <Button variant="ghost" on:click={props.sort.toggle}>
                      <Render of={cell.render()} />
                    </Button>
                  {:else}
                    <Render of={cell.render()} />
                  {/if}
                </Table.Head>
              </Subscribe>
            {/each}
          </Table.Row>
        </Subscribe>
      {/each}
    </Table.Header>
    <Table.Body
      {...$tableBodyAttrs}
      class="select-text selection:text-primary-foreground"
    >
      {#each $pageRows as row (row.id)}
        <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
          <Table.Row {...rowAttrs}>
            {#each row.cells as cell (cell.id)}
              <Subscribe attrs={cell.attrs()} let:attrs>
                <Table.Cell {...attrs}>
                  {#if cell.id === "type"}
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
  <div class="flex items-center justify-end space-x-4 py-4">
    <Button
      variant="outline"
      size="sm"
      on:click={() => ($pageIndex = $pageIndex - 1)}
      disabled={!$hasPreviousPage}>Previous</Button
    >
    <Button
      variant="outline"
      size="sm"
      disabled={!$hasNextPage}
      on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
    >
  </div>
</div>
