<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import type { User } from "$lib/types";
  import DataTable from "./data-table.svelte";
  import { writable, type Writable } from "svelte/store";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";

  export const users$: Writable<User[]> = writable([]);

  let unlisten: UnlistenFn

  onMount(async () => {
    // Get users from state.
    let users = await invoke<User[]>("get_users");
    updateUsers(users);

    unlisten = await listen<User[]>("users_update", (event) => {
      updateUsers(event.payload)
    })

    // Need a listener for when users is updated.
  })

  onDestroy(() => {
    unlisten?.()
  })

  function updateUsers (users: User[]) {
    users.sort((a,b) => a.username.localeCompare(b.username));

    users$.set(users);
  }

</script>

<h1>Users</h1>
<DataTable usersStore={users$}/>
