<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onDestroy, onMount } from "svelte";
  import type { User } from "$lib/types";
  import DataTable from "./data-table.svelte";
  import { writable, type Writable } from "svelte/store";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";

  export const users$: Writable<User[]> = writable([]);

  let unlisten: UnlistenFn;

  onMount(async () => {
    // Get users from state.
    let users = await invoke<User[]>("get_users");
    updateUsers(users);

    unlisten = await listen<User[]>("users_update", (event) => {
      updateUsers(event.payload);
    });
  });

  onDestroy(() => {
    unlisten?.();
  });

  function updateUsers(users: User[]) {
    users.sort((a, b) =>
      new Date(a.last_seen) < new Date(b.last_seen) ? 1 : -1,
    );

    users$.set(users);
  }
</script>

<h1>Users</h1>
<DataTable usersStore={users$} />
