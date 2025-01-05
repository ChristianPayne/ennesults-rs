<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import type { AuthValidation } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { start, cancel, onUrl } from "@fabianlars/tauri-plugin-oauth";

  let authStatus: AuthValidation;

  onMount(async () => {
    authStatus = await invoke<AuthValidation>("get_auth_status");
  });

  let port: number = undefined;
  let unlisten: () => void = undefined;
  let tryParse = true;

  async function startOAuthFlow() {
    try {
      port = await start({
        ports: [4500, 4501, 4502],
        response:
          "Thank you for authenticating Ennesults! You can close this window now.",
      });
      tryParse = true;
      console.log(`OAuth server started on port ${port}`);

      // Set up listeners for OAuth results
      unlisten = await onUrl(async (url) => {
        if (port && tryParse) {
          tryParse = false;
          // console.log("Received OAuth URL:", url);
          // Handle the OAuth redirect
          authStatus = await invoke("decode_auth_redirect", { url });
          stopOAuthServer();
        }
      });

      // Initiate your OAuth flow here
      await invoke("open_auth_window");
    } catch (error) {
      console.error("Error starting OAuth server:", error);
    }
  }

  // Don't forget to stop the server when you're done
  async function stopOAuthServer() {
    try {
      await cancel(port);
      console.log("OAuth server stopped");
      port = undefined;
      unlisten();
    } catch (error) {
      console.error("Error stopping OAuth server:", error);
    }
  }

  async function signOutOfTwitch() {
    authStatus = await invoke("sign_out_of_twitch");
  }
</script>

<div class="ml-8">
  <!-- {JSON.stringify(authStatus)} -->

  {#if authStatus?.["Valid"]}
    <Button on:click={signOutOfTwitch} variant="destructive"
      >Disconnect from Twitch</Button
    >
  {:else if authStatus?.["Invalid"]}
    <p>Invalid Authentication from Twitch!</p>
    <Button on:click={signOutOfTwitch} variant="destructive"
      >Disconnect from Twitch</Button
    >
  {:else}
    <Button on:click={startOAuthFlow}>Connect to Twitch</Button>
  {/if}
</div>
