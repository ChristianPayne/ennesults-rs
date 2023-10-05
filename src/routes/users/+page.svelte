<script lang="ts">
  import { Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, TableSearch } from 'flowbite-svelte';
  let searchTerm = '';
  let items = [
    { id: 1, user: 'ChrisGriffin522', type: 'Moderator', consented: true },
    { id: 2, user: 'Ennegineer', type: 'Streamer', consented: true },
    { id: 3, user: 'JinJix', type: 'Viewer', consented: false },
    { id: 4, user: 'jaypez04', type: 'Viewer', consented: false }
  ];
  $: filteredItems = items.filter((item) => item.user.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1);
</script>
<TableSearch placeholder="Search by user name" hoverable={true} bind:inputValue={searchTerm}>
  <TableHead>
    <TableHeadCell>ID</TableHeadCell>
    <TableHeadCell>Username</TableHeadCell>
    <TableHeadCell>Type</TableHeadCell>
    <TableHeadCell>Consented</TableHeadCell>
  </TableHead>
  <TableBody>
    {#each filteredItems as item}
      <TableBodyRow>
        <TableBodyCell>{item.id}</TableBodyCell>
        <TableBodyCell>{item.user}</TableBodyCell>
        <TableBodyCell>{item.type}</TableBodyCell>
        <TableBodyCell>{item.consented}</TableBodyCell>
      </TableBodyRow>
    {/each}
  </TableBody>
</TableSearch>