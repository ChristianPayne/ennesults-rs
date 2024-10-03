<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import Switch from "$lib/components/ui/switch/switch.svelte";
  import * as Select from "$lib/components/ui/select";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
    import { Checkbox } from "$lib/components/ui/checkbox";
  
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
  } as const;
  const connectionTypes: Selected<string>[] = Object.values(connectionTypeMap)

  // State values
  let autoConnectOnStartup: boolean = true;
  let channelName: string = "";
  let selectedConnectionType: Selected<string> = connectionTypeMap['anonymous'];
  let botName: string = "";
  let oauthTokenValue: string = "";

  onMount(async () => {
    let botInfo: any = await invoke("get_bot_info");
    channelName = botInfo.channel_name;
    botName = botInfo.bot_name;
    oauthTokenValue = botInfo.oauth_token;
    autoConnectOnStartup = botInfo.auto_connect_on_startup;

    if(botName && oauthTokenValue) {
      selectedConnectionType = connectionTypeMap['oauth']
    }
    console.log(botInfo)
  })

  function onConnectionTypeChanged(event: {value: string, label: string, disabled: boolean}) {
    console.log(event)
    selectedConnectionType = connectionTypeMap[event.value];
  }

  function onAutoConnectChanged(value: boolean) {
    console.log(value)
    autoConnectOnStartup = value;
  }

  async function save () {
    await invoke("leave_channel");
    let result = await invoke("save_bot_info", {
      botInfo: {
        channel_name: channelName,
        bot_name: botName,
        oauth_token: oauthTokenValue,
        auto_connect_on_startup: autoConnectOnStartup
      }
    })
    console.table({autoConnectOnStartup, channelName, selectedConnectionType:selectedConnectionType.value, botName, oauthTokenValue})
    console.log(result)
  }
</script>

<div class="flex flex-col">
  <h1>Settings</h1>
  <div class="ml-2 space-y-4">
    <div class="ml-2 px-4 space-y-2">
      <div class="space-y-1">
        <Checkbox checked={autoConnectOnStartup} onCheckedChange={onAutoConnectChanged} />
        <Label>Auto-connect on Startup</Label>
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
    <h2 class="text-2xl">Insults</h2>
    <div class="ml-2 px-4 space-y-2">
      <div class="space-y-1">
        <Checkbox checked={true} onCheckedChange={() => true} />
        <Label>Enable Insults</Label>
        <p class="text-sm text-muted-foreground">Enables insults to be said in chat by Ennesults.</p>
      </div>
      <div>
        <Label>Minimum users</Label>
        <p class="text-sm text-muted-foreground">Minimum users in chat to say an insult. Helps with not insulting the same people over and over again.</p>
        <Input type="number" placeholder="3"/>
      </div>
      <div>
        <Label for="insult-interval">Interval</Label>
        <p class="text-sm text-muted-foreground">How long, in seconds, between insults said in chat.</p>
        <Input id="insult-interval" type="number" placeholder="300"/>
      </div>
    </div>
    <h2 class="text-2xl">Comebacks</h2>
    <div class="ml-2 px-4 space-y-2">
      <div class="space-y-1">
        <Checkbox checked={true} onCheckedChange={() => true} />
        <Label>Enable Comebacks</Label>
        <p class="text-sm text-muted-foreground">Enables comebacks to be said in reply to people @-ing her.</p>
      </div>
    </div>
  </div>
</div>