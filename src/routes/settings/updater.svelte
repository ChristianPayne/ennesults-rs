<script lang="ts">
  import { Channel, invoke } from "@tauri-apps/api/core";
  import type { DownloadEvent } from "@tauri-apps/plugin-updater";
  import { exit, relaunch } from "@tauri-apps/plugin-process";
  import Button from "$lib/components/ui/button/button.svelte";

  let updateAvailable = false;
  let updateButtonDisabled = false;
  let checkForUpdateButtonMessage = "Check for Update";
  let pendingRestart = false;
  async function checkForUpdate() {
    updateButtonDisabled = true;
    if (updateAvailable === false) {
      checkForUpdateButtonMessage = "Checking!";
      let result = await invoke<{
        version: string;
        currentVersion: string;
      } | null>("fetch_update");
      if (result === null) {
        checkForUpdateButtonMessage = "Up to date!";
        setTimeout(() => {
          checkForUpdateButtonMessage = "Check for Update";
          updateButtonDisabled = false;
        }, 3000);
      } else {
        updateAvailable = true;
        checkForUpdateButtonMessage = `Update to v.${result.version}!`;
        updateButtonDisabled = false;
      }
    } else {
      const onEvent = new Channel<DownloadEvent>();
      onEvent.onmessage = (message) => {
        switch (message.event) {
          case "Started": {
            checkForUpdateButtonMessage = `Starting update...`;
            break;
          }
          case "Progress": {
            checkForUpdateButtonMessage = `Installing update...`;
            break;
          }
          case "Finished": {
            pendingRestart = true;
            break;
          }
        }
      };

      let updater = await invoke("install_update", { onEvent });
    }
  }

  async function restart() {
    await relaunch();
  }
</script>

{#if !pendingRestart}
  <Button on:click={checkForUpdate} disabled={updateButtonDisabled}
    >{checkForUpdateButtonMessage}</Button
  >
{:else}
  <Button on:click={restart}>Restart</Button>
{/if}
