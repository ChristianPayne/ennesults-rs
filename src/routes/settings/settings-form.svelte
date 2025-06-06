<script lang="ts">
  import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";
  import * as Form from "$lib/components/ui/form";
  import * as Select from "$lib/components/ui/select";
  import { Input } from "$lib/components/ui/input";
  import { formSchema, type FormSchema } from "./schema";
  import {
    type SuperValidated,
    type Infer,
    type SuperForm,
    superForm,
  } from "sveltekit-superforms";
  import { zodClient } from "sveltekit-superforms/adapters";
  import Label from "$lib/components/ui/label/label.svelte";

  export let validatedForm: SuperValidated<any, any, any>;
  export let onUpdated: (event: {
    form: Readonly<SuperValidated<any, any, any>>;
  }) => unknown;

  let form: SuperForm<Infer<FormSchema>> = superForm(validatedForm, {
    clearOnSubmit: "none",
    SPA: true,
    validators: zodClient(formSchema),
    resetForm: false,
    onUpdated,
  });

  const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance class="space-y-4">
  <div class="md:ml-8">
    <Form.Field {form} name="autoConnectOnStartup">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.autoConnectOnStartup} />
          <input
            name={attrs.name}
            bind:value={$formData.autoConnectOnStartup}
            hidden
          />
          <Form.Label>Auto-connect on Startup</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Should the bot join your channel automatically?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="channelName">
      <Form.Control let:attrs>
        <Form.Label>Channel Name</Form.Label>
        <Input {...attrs} bind:value={$formData.channelName} />
      </Form.Control>
      <Form.Description
        >What channel do you want the bot to join?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>

    <!-- <Form.Field {form} name="botName">
      <Form.Control let:attrs>
        <Form.Label class="block mb-2">Bot name</Form.Label>
        <Input
          {...attrs}
          class="placeholder:text-muted"
          placeholder="Ennesults"
          type="text"
          bind:value={$formData.botName}
        />
      </Form.Control>
      <Form.Description
        >What is the name of the bot account you want to use?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="oauthTokenValue">
      <Form.Control let:attrs>
        <Form.Label class="block mb-2">OAuth token</Form.Label>
        <Input
          {...attrs}
          type="password"
          class="placeholder:text-muted"
          placeholder="01J924W48ACP2FDDR7Y6FW88PQ"
          bind:value={$formData.oauthTokenValue}
        />
      </Form.Control>
      <Form.Description
        >What is the oAuth token of the bot account?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field> -->
  </div>

  <h2>Announcements</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="enableAnnouncements">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.enableAnnouncements} />
          <input
            name={attrs.name}
            bind:value={$formData.enableAnnouncements}
            hidden
          />
          <Form.Label>Enable Announcements</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Enables announcements to be said in chat by the bot.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="minimumTimeBetweenAnnouncements">
      <Form.Control let:attrs>
        <Form.Label>Minimum Time Between Announcements</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.minimumTimeBetweenAnnouncements}
        />
      </Form.Control>
      <Form.Description
        >The minimum amount of time (seconds) between announcements.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="maximumTimeBetweenAnnouncements">
      <Form.Control let:attrs>
        <Form.Label>Maximum Time Between Announcements</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.maximumTimeBetweenAnnouncements}
        />
      </Form.Control>
      <Form.Description
        >The maximum amount of time (seconds) between announcements.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="randomizeAnnouncements">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox
            {...attrs}
            bind:checked={$formData.randomizeAnnouncements}
          />
          <input
            name={attrs.name}
            bind:value={$formData.randomizeAnnouncements}
            hidden
          />
          <Form.Label>Randomize Announcements</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Chooses whether or not to pick a random announcement every time or go
        sequentially.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
  </div>

  <h2>Insults</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="enableInsults">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.enableInsults} />
          <input
            name={attrs.name}
            bind:value={$formData.enableInsults}
            hidden
          />
          <Form.Label>Enable Insults</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Enables insults to be said in chat by the bot.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="minimumTimeBetweenInsults">
      <Form.Control let:attrs>
        <Form.Label>Minimum Time Between Insults</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.minimumTimeBetweenInsults}
        />
      </Form.Control>
      <Form.Description
        >The minimum amount of time (seconds) between insults.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="maximumTimeBetweenInsults">
      <Form.Control let:attrs>
        <Form.Label>Maximum Time Between Insults</Form.Label>
        <Input {...attrs} type="number" bind:value={$formData.maximumTimeBetweenInsults} />
      </Form.Control>
      <Form.Description
        >The maximum amount of time (seconds) between insults.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="lurkTime">
      <Form.Control let:attrs>
        <Form.Label>Lurk Time</Form.Label>
        <Input {...attrs} type="number" bind:value={$formData.lurkTime} />
      </Form.Control>
      <Form.Description
        >How long (minutes) since someone chatted until we flag them as lurking.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
  </div>

  <h2>Comebacks</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="enableComebacks">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.enableComebacks} />
          <input
            name={attrs.name}
            bind:value={$formData.enableComebacks}
            hidden
          />
          <Form.Label>Enable Comebacks</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Enables comebacks to be said in reply to people @-ing her.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="comebackExceptions">
      <Form.Control let:attrs>
        <Form.Label>Comeback Exceptions</Form.Label>
        <Input {...attrs} bind:value={$formData.comebackExceptions} />
      </Form.Control>
      <Form.Description
        >Exceptions for people that may abuse @-ing Ennesults (comma separated
        usernames).</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="percentChanceOfComeback">
      <Form.Control let:attrs>
        <Form.Label>Percent Chance of Comeback</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.percentChanceOfComeback}
        />
      </Form.Control>
      <Form.Description
        >Replying every time would get tiring. What percent (%) should we snap
        back?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
  </div>

  <h2>Corrections</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="enableCorrections">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.enableCorrections} />
          <input
            name={attrs.name}
            bind:value={$formData.enableCorrections}
            hidden
          />
          <Form.Label>Enable Corrections</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Enables Ennesults to correct people misspelling "Enne".</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="correctionExceptions">
      <Form.Control let:attrs>
        <Form.Label>Correction Exceptions</Form.Label>
        <Input {...attrs} bind:value={$formData.correctionExceptions} />
      </Form.Control>
      <Form.Description
        >Parts of a word that don't make sense to correct (comma separated).</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="percentChanceOfCorrection">
      <Form.Control let:attrs>
        <Form.Label>Percent Chance of Correction</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.percentChanceOfCorrection}
        />
      </Form.Control>
      <Form.Description
        >What percent (%) of the time should we correct viewers?</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
  </div>

  <h2>Whispers</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="enableWhispers">
      <Form.Control let:attrs>
        <div class="flex items-center space-x-2">
          <Checkbox {...attrs} bind:checked={$formData.enableWhispers} />
          <input
            name={attrs.name}
            bind:value={$formData.enableWhispers}
            hidden
          />
          <Form.Label>Enable Whispers</Form.Label>
        </div>
      </Form.Control>
      <Form.Description
        >Enables the bot to say in chat what users whisper to her.</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
    <Form.Field {form} name="usersAllowedToWhisper">
      <Form.Control let:attrs>
        <Form.Label>Users Allowed to Whisper</Form.Label>
        <Input {...attrs} bind:value={$formData.usersAllowedToWhisper} />
      </Form.Control>
      <Form.Description
        >Each user's name that can whisper (comma separated).</Form.Description
      >
      <Form.FieldErrors />
    </Form.Field>
  </div>

  <h2>Message Queue</h2>
  <div class="md:ml-8">
    <Form.Field {form} name="messageQueueInterval">
      <Form.Control let:attrs>
        <Form.Label>Message Queue Interval</Form.Label>
        <Input
          {...attrs}
          type="number"
          bind:value={$formData.messageQueueInterval}
        />
      </Form.Control>
      <Form.Description>
        The amount of time (seconds) between messages sent from insults and announcements.
      </Form.Description>
    </Form.Field>
  </div>

  <div class="flex justify-center">
    <Form.Button class="w-1/3">Save</Form.Button>
  </div>
</form>
