<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { User } from "$lib/types";
  import DataTable from "./data-table.svelte";
  import { writable, type Writable } from "svelte/store";

  export const users$: Writable<User[]> = writable([]);

  onMount(async () => {
    // Get users from state.
    let result = await invoke<User[]>("get_users");
    result.sort((a,b) => a.username.localeCompare(b.username));

    users$.set(result);

    // Need a listener for when users is updated.
  })

</script>

<h1>Users</h1>
<DataTable usersStore={users$}/>
