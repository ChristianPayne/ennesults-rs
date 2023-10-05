<script lang="ts">
  import { Input, Label, Toggle } from "flowbite-svelte";

</script>

<div class="flex flex-col space-y-4">
  <Label for="channel-name" class="block mb-2">
    Channel Name
    <Input id="channel-name" placeholder="Ennegineer" type="text" />
  </Label>
  
  <Toggle>Toggle me</Toggle>
</div>