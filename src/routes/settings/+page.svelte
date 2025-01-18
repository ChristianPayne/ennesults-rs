<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import * as Select from "$lib/components/ui/select";
  import { onMount } from "svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import type { BotInfo, DownloadEvent } from "$lib/types";
  import { toast } from "svelte-sonner";
  import { colorPalettes } from "$lib/colorPalettes";
  import { theme, setTheme, toggleMode } from "mode-watcher";
  import SettingsForm from "./settings-form.svelte";
  import { formSchema, type FormSchema } from "./schema";

  import {
    superValidate,
    type SuperValidated,
    type Infer,
  } from "sveltekit-superforms";
  import { zod } from "sveltekit-superforms/adapters";
  import ConnectToTwitch from "./connect-to-twitch.svelte";
  import Updater from "./updater.svelte";

  let validatedForm: SuperValidated<any, any, any>;

  onMount(async () => {
    const botInfo = await invoke<BotInfo>("get_bot_info");
    const usersAllowedToWhisperResult = await invoke<string[]>(
      "get_users_allowed_to_whisper",
    );

    let settings = {
      autoConnectOnStartup: botInfo.auto_connect_on_startup,
      channelName: botInfo.channel_name,
      enableWhispers: botInfo.enable_whispers,
      usersAllowedToWhisper: usersAllowedToWhisperResult.join(", "),
      enableAnnouncements: botInfo.enable_announcements,
      timeBetweenAnnouncements: botInfo.time_between_announcements,
      randomizeAnnouncements: botInfo.randomize_announcements,
      enableInsults: botInfo.enable_insults,
      timeBetweenInsults: botInfo.time_between_insults,
      lurkTime: botInfo.lurk_time,
      enableComebacks: botInfo.enable_comebacks,
      percentChanceOfComeback: botInfo.percent_chance_of_comeback,
      enableCorrections: botInfo.enable_corrections,
      percentChanceOfCorrection: botInfo.percent_chance_of_correction,
      correctionExceptions: botInfo.correction_exceptions.join(", "),
    };

    validatedForm = await superValidate(settings, zod(formSchema));
  });

  async function onFormUpdate(event: {
    form: Readonly<SuperValidated<any, any, any>>;
  }) {
    let { form: f } = event;
    if (f.valid === false) {
      toast.error(
        Object.values(f.errors)
          .flatMap((v) => v)
          .join("; "),
      );
      return;
    }

    // Save info from validated form.
    await save(f.data as Infer<FormSchema>);
  }

  async function save(validatedData: Infer<FormSchema>) {
    toast.info("Saving settings...");

    await invoke<string>("leave_channel");

    await invoke<BotInfo>("save_bot_info", {
      botInfo: {
        channel_name: validatedData.channelName,
        // bot_name: validatedData.botName,
        // oauth_token: validatedData.oauthTokenValue,
        auto_connect_on_startup: validatedData.autoConnectOnStartup,
        enable_whispers: validatedData.enableWhispers,
        users_allowed_to_whisper: validatedData.usersAllowedToWhisper
          .trim()
          .split(",")
          .filter(Boolean)
          .map((user) => user.trim().toLowerCase()),
        enable_announcements: validatedData.enableAnnouncements,
        time_between_announcements: validatedData.timeBetweenAnnouncements,
        randomize_announcements: validatedData.randomizeAnnouncements,
        enable_insults: validatedData.enableInsults,
        time_between_insults: validatedData.timeBetweenInsults,
        lurk_time: validatedData.lurkTime,
        enable_comebacks: validatedData.enableComebacks,
        percent_chance_of_comeback: validatedData.percentChanceOfComeback,
        enable_corrections: validatedData.enableCorrections,
        percent_chance_of_correction: validatedData.percentChanceOfCorrection,
        comeback_exceptions: validatedData.comebackExceptions
          .trim()
          .split(",")
          .filter(Boolean)
          .map((user) => user.trim().toLowerCase()),
        correction_exceptions: validatedData.correctionExceptions
          .trim()
          .split(",")
          .filter(Boolean)
          .map((user) => user.trim().toLowerCase()),
      },
    })
      .then((botInfo) => {
        toast.info("Saved settings!");
        console.log(botInfo);
      })
      .catch((e) => {
        toast.error("Error saving settings...");
      });
  }

  function onColorPaletteChange(value: any) {
    console.log("onColorPaletteChange", value);
    setTheme(value.value);
  }

  function getCurrentColorPalette() {
    if (colorPalettes[$theme] !== undefined) {
      return colorPalettes[$theme];
    } else {
      setTheme("");
      return colorPalettes["ennesults"];
    }
  }
</script>

<h1 class="mb-4">Settings</h1>
<div class="ml-2 space-y-6">
  <div class="flex items-end space-x-2">
    <div class="md:ml-8">
      <Label>Theme</Label>
      <Select.Root
        selected={getCurrentColorPalette()}
        onSelectedChange={onColorPaletteChange}
      >
        <Select.Trigger class="w-[180px]">
          <Select.Value placeholder="Color Palette" />
        </Select.Trigger>
        <Select.Content>
          {#each Object.keys(colorPalettes) as paletteKey}
            <Select.Item value={colorPalettes[paletteKey].value}
              >{colorPalettes[paletteKey].label}</Select.Item
            >
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
    <Button on:click={toggleMode} variant="outline" size="icon">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="size-5"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
        />
      </svg>
    </Button>
    <!-- UPDATER -->
    <Updater />
  </div>

  <ConnectToTwitch />

  {#if validatedForm}
    <SettingsForm {validatedForm} onUpdated={onFormUpdate} />
  {/if}
</div>
