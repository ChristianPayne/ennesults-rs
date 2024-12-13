<script lang="ts">
  import type { Announcement } from "$lib/types";
  import { derived, get, writable, type Writable } from "svelte/store";
  import * as Table from "$lib/components/ui/table";
  import {
    createTable,
    Render,
    Subscribe,
    createRender,
  } from "svelte-headless-table";
  import DataTableActions from "./data-table-actions.svelte";
  import EditAnnouncement from "$lib/components/editAnnouncement.svelte";
  import { invoke } from "@tauri-apps/api/core";

  export let announcements: Writable<Announcement[]>;

  const table = createTable(announcements);

  let announcementBeingEdited: Writable<string> = writable("");

  function setAnnouncementBeingEdited(id: string) {
    announcementBeingEdited.set(id);
  }

  async function updateAnnouncement(announcementValue: string) {
    if (!get(announcementBeingEdited)) return;
    if (announcementValue == "") {
      announcementBeingEdited.set("");
      return;
    }
    await invoke("update_announcement", {
      announcement: {
        id: get(announcementBeingEdited),
        value: announcementValue,
      },
    });
    announcementBeingEdited.set("");
  }

  const columns = table.createColumns([
    // table.column({
    //   accessor: "id",
    //   header: "ID",
    // }),
    table.column({
      accessor: (announcement) => announcement,
      header: "Announcement",
      cell: ({ value }) =>
        createRender(
          EditAnnouncement,
          derived(announcementBeingEdited, (announcementBeingEdited) => ({
            announcementBeingEdited,
            announcement: value,
            callback: updateAnnouncement,
          })),
        ),
    }),
    table.column({
      accessor: "id",
      header: "Actions",
      cell: ({ value }) => {
        return createRender(DataTableActions, {
          id: value,
          setAnnouncementBeingEdited,
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
