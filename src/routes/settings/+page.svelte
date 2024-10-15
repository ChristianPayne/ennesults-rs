<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import Switch from "$lib/components/ui/switch/switch.svelte";
  import * as Select from "$lib/components/ui/select";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import type { BotInfo } from "$lib/types";
  import { toast } from "svelte-sonner";
  import { colorPalettes } from "$lib/colorPalettes";
  import { theme, setTheme, toggleMode } from "mode-watcher";
  
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
  let enableWhispers = false;
  let enableInsults = false;
  let enableComebacks = false;
  let enableCorrections = false;
  let usersAllowedToWhisper = "";
  let correctionExceptions = "";

  onMount(async () => {
    let botInfo = await invoke<BotInfo>("get_bot_info");
    channelName = botInfo.channel_name;
    botName = botInfo.bot_name;
    oauthTokenValue = botInfo.oauth_token;
    autoConnectOnStartup = botInfo.auto_connect_on_startup;

    enableWhispers = botInfo.enable_whispers;
    enableInsults = botInfo.enable_insults;
    enableComebacks = botInfo.enable_comebacks;

    let usersAllowedToWhisperResult = await invoke<string[]>("get_users_allowed_to_whisper");
    console.log('🪵 ~ onMount ~ usersAllowedToWhisperResult:', usersAllowedToWhisperResult);
    usersAllowedToWhisper = usersAllowedToWhisperResult.join(", ")

    if(botName && oauthTokenValue) {
      selectedConnectionType = connectionTypeMap['oauth']
    }
  })

  function onConnectionTypeChanged(event: {value: string, label: string, disabled: boolean}) {
    selectedConnectionType = connectionTypeMap[event.value];
  }

  function onAutoConnectChanged(value: boolean) {
    autoConnectOnStartup = value;
  }

  function onEnableWhisper (value: boolean) {
    enableWhispers = value
  }
  function onEnableInsults (value: boolean) {
    enableInsults = value
  }
  function onEnableComebacks (value: boolean) {
    enableComebacks = value
  }
  function onEnableCorrections (value: boolean) {
    enableCorrections = value
  }

  async function save () {
    toast.info("Saving settings...")
    await invoke<string>("leave_channel").catch(async e => {
      toast.info(e)
    });
    await invoke<BotInfo>("save_bot_info", {
      botInfo: {
        channel_name: channelName,
        bot_name: botName,
        oauth_token: oauthTokenValue,
        auto_connect_on_startup: autoConnectOnStartup,
        enable_whispers: enableWhispers,
        enable_insults: enableInsults,
        enable_comebacks: enableComebacks
      }
    })
    let saveUsersAllowedToWhisperResult = await invoke<string[]>("save_users_allowed_to_whisper", {
      usersAllowedToWhisper: usersAllowedToWhisper.split(",").filter(Boolean).map(user => user.trim().toLowerCase())
    })
  }

  function onColorPaletteChange (value: any) {
    console.log("onColorPaletteChange", value)
    setTheme(value.value)
  }

  function getCurrentColorPalette() {
    if(colorPalettes[$theme] !== undefined) {
      return colorPalettes[$theme]
    } else {
      setTheme("")
      return colorPalettes["ennesults"]
    }
  }
</script>

<div class="flex flex-col">
  <h1 class="mb-4">Settings</h1>
  <div class="ml-2 space-y-8">
    <div class="ml-2 px-4 space-y-4">
      <div class="flex items-end space-x-2">
        <div>
          <Label>Theme</Label>
          <Select.Root selected={getCurrentColorPalette()} onSelectedChange={onColorPaletteChange}>
            <Select.Trigger class="w-[180px]">
              <Select.Value placeholder="Color Palette" />
            </Select.Trigger>
            <Select.Content>
              {#each Object.keys(colorPalettes) as paletteKey}
                <Select.Item value={colorPalettes[paletteKey].value}>{colorPalettes[paletteKey].label}</Select.Item>
              {/each}
            </Select.Content>
          </Select.Root>
        </div>
        <Button on:click={toggleMode} variant="outline" size="icon">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
          </svg>
        </Button>
      </div>
      <div class="flex items-center space-x-2">
        <Checkbox checked={autoConnectOnStartup} onCheckedChange={onAutoConnectChanged} />
        <Label>Auto-connect on Startup</Label>
      </div>
      <div>
        <Label class="block mb-2">Channel name</Label>
        <Input placeholder="Ennegineer" type="text" bind:value={channelName} />
      </div>
      <div>
        <Label class="block mb-2">Connection type</Label>
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
          <Label class="block mb-2">Bot name</Label>
          <Input placeholder="Ennesults" type="text" bind:value={botName}/>
        </div>
        <div>
          <Label class="block mb-2">OAuth token</Label>
          <Input type="password" placeholder="01J924W48ACP2FDDR7Y6FW88PQ" bind:value={oauthTokenValue}/>
        </div>
      {/if}
    </div>
    <div class="ml-2 px-4 space-y-2">
      <h2 class="text-2xl">Whispers</h2>
      <div>
        <div class="flex items-center space-x-2">
          <Checkbox checked={enableWhispers} onCheckedChange={value => onEnableWhisper(Boolean(value))}/>
          <Label>Enable Whispers</Label>
        </div>
        <p class="text-sm text-muted-foreground">Enables Ennesults to say in chat what users whisper to her.</p>
      </div>
      <div>
        <Label>Users allowed to whisper</Label>
        <p class="text-sm text-muted-foreground">Each user's name that can whisper (comma separated).</p>
        <Input placeholder="chrisgriffin522" type="text" bind:value={usersAllowedToWhisper}/>
      </div>
    </div>
    <div class="ml-2 px-4 space-y-2">
      <h2 class="text-2xl">Insults</h2>
      <div class="flex flex-col gap-2">
        <div class="flex items-center space-x-2">
          <Checkbox checked={enableInsults} onCheckedChange={value => onEnableInsults(Boolean(value))}/>
          <Label>Enable Insults</Label>
        </div>
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

    <div class="ml-2 px-4 space-y-2">
      <h2 class="text-2xl">Comebacks</h2>
      <div class="flex flex-col gap-2">
        <div class="flex items-center space-x-2">
          <Checkbox checked={enableComebacks} onCheckedChange={value => onEnableComebacks(Boolean(value))}/>
          <Label>Enable Comebacks</Label>
        </div>
        <p class="text-sm text-muted-foreground">Enables comebacks to be said in reply to people @-ing her.</p>
      </div>
      <div>
        <Label>Comeback exceptions</Label>
        <p class="text-sm text-muted-foreground">Exceptions for people that may abuse @-ing Ennesults (comma separated).</p>
        <Input type="text" bind:value={correctionExceptions}/>
      </div>
      <div>
        <Label for="percent-correction">Percent chance of comeback</Label>
        <p class="text-sm text-muted-foreground">Replying every time would get tiring. What percent (%) should we snap back?</p>
        <Input id="percent-correction" type="number"/>
      </div>
    </div>

    <div class="ml-2 px-4 space-y-2">
      <h2 class="text-2xl">Corrections</h2>
      <div class="flex flex-col gap-2">
        <div class="flex items-center space-x-2">
          <Checkbox checked={enableCorrections} onCheckedChange={value => onEnableCorrections(Boolean(value))}/>
          <Label>Enable Corrections</Label>
        </div>
        <p class="text-sm text-muted-foreground">Enables Ennesults to correct people misspelling "Enne".</p>
      </div>
      <div>
        <Label>Correction exceptions</Label>
        <p class="text-sm text-muted-foreground">Parts of a word that don't make sense to correct (comma separated).</p>
        <Input type="text" bind:value={correctionExceptions}/>
      </div>
      <div>
        <Label for="percent-correction">Percent chance of correction</Label>
        <p class="text-sm text-muted-foreground">Correcting every time would get tiring. What percent (%) should we correct someone?</p>
        <Input id="percent-correction" type="number"/>
      </div>
    </div>
    <div class="flex w-full justify-center">
      <Button on:click={save} class="w-1/3">Save</Button>
    </div>
  </div>
</div>