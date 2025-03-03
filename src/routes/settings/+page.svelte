<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import Input from "$lib/components/ui/input/input.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
  import * as Select from "$lib/components/ui/select";
  import { onMount } from "svelte";
  import { invoke, Channel } from "@tauri-apps/api/core";
  import type { Settings, DownloadEvent } from "$lib/types";
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
    const settings = await invoke<Settings>("get_settings");
    const usersAllowedToWhisperResult = await invoke<string[]>(
      "get_users_allowed_to_whisper",
    );

    let formattedSettings = {
      autoConnectOnStartup: settings.auto_connect_on_startup,
      channelName: settings.channel_name,
      enableWhispers: settings.enable_whispers,
      usersAllowedToWhisper: usersAllowedToWhisperResult.join(", "),
      enableAnnouncements: settings.enable_announcements,
      maximumTimeBetweenAnnouncements: settings.maximum_time_between_announcements,
      minimumTimeBetweenAnnouncements: settings.minimum_time_between_announcements,
      randomizeAnnouncements: settings.randomize_announcements,
      enableInsults: settings.enable_insults,
      maximumTimeBetweenInsults: settings.maximum_time_between_insults,
      minimumTimeBetweenInsults: settings.minimum_time_between_insults,
      lurkTime: settings.lurk_time,
      enableComebacks: settings.enable_comebacks,
      percentChanceOfComeback: settings.percent_chance_of_comeback,
      enableCorrections: settings.enable_corrections,
      percentChanceOfCorrection: settings.percent_chance_of_correction,
      correctionExceptions: settings.correction_exceptions.join(", "),
    };

    validatedForm = await superValidate(formattedSettings, zod(formSchema));
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

    await invoke<Settings>("save_settings", {
      settings: {
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
        maximum_time_between_announcements: validatedData.maximumTimeBetweenAnnouncements,
        minimum_time_between_announcements: validatedData.minimumTimeBetweenAnnouncements,
        randomize_announcements: validatedData.randomizeAnnouncements,
        enable_insults: validatedData.enableInsults,
        maximum_time_between_insults: validatedData.maximumTimeBetweenInsults,
        minimum_time_between_insults: validatedData.minimumTimeBetweenInsults,
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
      .then((settings) => {
        toast.info("Saved settings!");
        console.log(settings);
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
