<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import Switch from "$lib/components/ui/switch/switch.svelte";
  import * as Select from "$lib/components/ui/select";
  import Title from "$lib/Title.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  
  type Selected<Value> = {
    value: Value;
    label?: string;
  };

  const connectionTypeMap: {[connectionTypeValue: string]: Selected<string>} = {
    "anonymous": {
      value: "anonymous",
      label: "Anonymous"
    },
    "oauth": {
      value: "oauth",
      label: "OAuth"
    }
  }
  const connectionTypes: Selected<string>[] = Object.values(connectionTypeMap)

  // State values
  let autoConnectOnStartup: boolean = true;
  let channelName: string = "";
  let selectedConnectionType: Selected<string> = connectionTypeMap['anonymous'];
  let botName: string = "";
  let oauthTokenValue: string = "";

  onMount(async () => {
    channelName = await invoke("get_channel_name");
  })

  function onConnectionTypeChanged(event: {value: string, label: string, disabled: boolean}) {
    console.log(event)
    selectedConnectionType = connectionTypeMap[event.value];
  }

  function onAutoConnectChanged(value: boolean) {
    console.log(value)
    autoConnectOnStartup = value;
  }

  function save () {
    console.table({autoConnectOnStartup, channelName, selectedConnectionType:selectedConnectionType.value, botName, oauthTokenValue})
  }
</script>

<div class="flex flex-col space-y-4">
  <Title title="Settings"/>
  <div class="px-4 space-y-2">
    <div>
      <Label class="block mb-2">Auto-connect on Startup</Label>
      <Switch checked={autoConnectOnStartup} onCheckedChange={onAutoConnectChanged} />
    </div>
    <div>
      <Label class="block mb-2">Channel Name</Label>
      <Input placeholder="Ennegineer" type="text" bind:value={channelName} />
    </div>
    <div>
      <Label class="block mb-2">Connection Type</Label>
      <Select.Root selected={selectedConnectionType} onSelectedChange={onConnectionTypeChanged}>
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Connection Type" />
        </Select.Trigger>
        <Select.Content>
          {#each connectionTypes as connectionType}
            <Select.Item value={connectionType.value}>{connectionType.label}</Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
    {#if selectedConnectionType.value === "oauth"}
      <div>
        <Label class="block mb-2">Bot Name</Label>
        <Input placeholder="Ennesults" type="text" bind:value={botName}/>
      </div>
      <div>
        <Label class="block mb-2">OAuth Token</Label>
        <Input type="password" placeholder="01J924W48ACP2FDDR7Y6FW88PQ" bind:value={oauthTokenValue}/>
      </div>
    {/if}
    
    <Button on:click={save}>Save</Button>
  </div>
  
</div>