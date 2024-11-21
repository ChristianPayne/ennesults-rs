<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "svelte-sonner";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Button } from "$lib/components/ui/button/index.js";

  let speakAsEnnesultsMessage = "";
  let speakAsEnnesultsDialog: boolean = false;

  export let connectionStatus: boolean;

  async function speakAsEnnesults() {
    if (speakAsEnnesultsMessage === "") return;
    if (connectionStatus === false) return;

    let result = await invoke("say", {
      message: speakAsEnnesultsMessage,
    }).catch((e) => {
      toast.error("Something went wrong!", {
        description: "Failed to send chat message!" + e,
      });
    });

    speakAsEnnesultsMessage = "";
    speakAsEnnesultsDialog = false;
  }

  function onKeyDown(e) {
    switch (e.keyCode) {
      case 13: {
        // Enter
        speakAsEnnesults();
        break;
      }
      case 27: {
        // Escape
        speakAsEnnesultsMessage = "";
        speakAsEnnesultsDialog = false;
        break;
      }
    }
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div on:keydown={onKeyDown}>
  <Dialog.Root bind:open={speakAsEnnesultsDialog}>
    <Dialog.Trigger class="text-sm">
      <Button class="py-0" variant="default">Send Message</Button>
    </Dialog.Trigger>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>Make Ennesults speak in chat!</Dialog.Title>
        <Dialog.Description>
          <div class="grid w-full gap-1.5">
            <Textarea
              bind:value={speakAsEnnesultsMessage}
              placeholder="Type your message here."
            />
            <Button on:click={speakAsEnnesults}>Send</Button>
          </div>
        </Dialog.Description>
      </Dialog.Header>
    </Dialog.Content>
  </Dialog.Root>
</div>
