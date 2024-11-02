<script lang="ts">
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button";
  
  export let id: string;
 </script>
  
 <DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
   <Button
    variant="ghost"
    builders={[builder]}
    size="icon"
    class="relative h-8 w-8 p-0"
   >
    <span class="sr-only">Open menu</span>
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
      <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 12.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5ZM12 18.75a.75.75 0 1 1 0-1.5.75.75 0 0 1 0 1.5Z" />
    </svg>
    
   </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content>
   <DropdownMenu.Group>
    <DropdownMenu.Label>Actions</DropdownMenu.Label>
    <DropdownMenu.Item on:click={() => navigator.clipboard.writeText(id.toString())}>
     Copy ID
    </DropdownMenu.Item>
   </DropdownMenu.Group>
   <DropdownMenu.Separator />
   <DropdownMenu.Item>Toggle consent</DropdownMenu.Item>
   <DropdownMenu.Item>Allow whispers</DropdownMenu.Item>
   <DropdownMenu.Item class="text-destructive">Delete user</DropdownMenu.Item>
  </DropdownMenu.Content>
 </DropdownMenu.Root>