<script lang="ts">
  import type { Insult } from "$lib/types";
  import { Badge } from "$lib/components/ui/badge";

  export let insult: Insult;
</script>

<div class="flex space-x-2">
  {#each insult.tags as tag}
    <Badge variant="outline">{tag}</Badge>
  {/each}
</div>
