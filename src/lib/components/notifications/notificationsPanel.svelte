<script lang="ts">
  import * as Sheet from "$lib/components/ui/sheet/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { markAllAsSeen, markNotificationAsSeen, notifications$, type Notification } from "./notifications";

  let notifications: Notification[] = [];
  notifications$.subscribe(update => {
    notifications = update.filter(n => n.seen === false)
  });
</script>


<Sheet.Root>
  <Sheet.Trigger asChild let:builder>
    <Button builders={[builder]} variant="ghost" class="relative">
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1" stroke="currentColor" class="size-5">
        <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" />
      </svg>
      {#if notifications.length > 0}
        <div class="absolute top-1 right-2 bg-destructive rounded-full h-2 w-2 flex items-center justify-center"></div>
      {/if}
    </Button>
  </Sheet.Trigger>
  <Sheet.Content side="right" class="flex flex-col gap-2">
    <Sheet.Header>
      <Sheet.Title>Notifications</Sheet.Title>
      <Sheet.Description>
        See past notifications here.
      </Sheet.Description>
    </Sheet.Header>

    <div class="flex flex-col gap-2 h-full overflow-y-scroll ">
      {#if notifications.length > 0 }
        {#each notifications as notification}
        <div class="bg-muted rounded-md p-2">
          <div class="flex justify-between">
            <h3>{notification.title}</h3>
            <button on:click={() => markNotificationAsSeen(notification.id)} class="rounded-sm focus:outline-none">
              <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-4">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
          {#if notification?.description}
            <p class="font-thin">{notification?.description}</p>
          {/if}
        </div>
        {/each}
      {:else}
        <div class="flex items-center justify-center grow">
          You are up to date!
        </div>
      {/if}
    </div>

    <Sheet.Footer>
      <Sheet.Close asChild let:builder>
        {#if notifications.length > 0 }
          <Button builders={[builder]} type="submit" on:click={markAllAsSeen}>Clear all notifications</Button>
        {/if}
      </Sheet.Close>
    </Sheet.Footer>
  </Sheet.Content>
</Sheet.Root>