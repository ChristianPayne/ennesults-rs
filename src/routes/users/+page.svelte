<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { User } from "$lib/types";
  import DataTable from "./data-table.svelte";

  let users: User[] = [
    {
      consented: false,
      id: "test",
      last_seen: "date",
      username: "name"
    }
  ];

  onMount(async () => {
    // Get users from state.
    users = await invoke<User[]>("get_users");
    // users.sort((a,b) => a.username.localeCompare(b.username));

    // console.log('🪵 ~ onMount ~ users:', users);

    // Need a listener for when users is updated.
  })

</script>

<h1>Users</h1>
<DataTable data={users}/>
