<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import Switch from "$lib/components/ui/switch/switch.svelte";
  import * as Select from "$lib/components/ui/select";

  function selected(event: any) {
    console.log(event)
  }

</script>

<div class="flex flex-col space-y-4">
  <h1 class="text-4xl">Settings</h1>
  <div class="px-4 space-y-2">
    <div>
      <Label class="block mb-2">Auto-connect on Startup</Label>
      <Switch/>
    </div>
    <div>
      <Label class="block mb-2">Connection Type</Label>
      <Select.Root onSelectedChange={selected}>
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Connection Type" />
        </Select.Trigger>
        <Select.Content>
          <Select.Item value="anonymous">Anonymous</Select.Item>
          <Select.Item value="oauth">OAuth</Select.Item>
        </Select.Content>
      </Select.Root>
    </div>
    <div>
      <Label class="block mb-2">Channel Name</Label>
      <Input placeholder="Ennegineer" type="text" />
    </div>
    <div>
      <Label class="block mb-2">Bot Name</Label>
      <Input placeholder="Ennesults" type="text" />
    </div>
    <div>
      <Label class="block mb-2">OAuth Token</Label>
      <Input type="password" placeholder="58f50999-79cf-47fc-85f4-d0fd929284c8" />
    </div>
    
    <Button>Save</Button>
  </div>
  
</div>