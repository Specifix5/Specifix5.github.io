<script lang="ts">
  import { onMount } from "svelte";
  import { DiscordInviteLink } from "../scripts/utils";

  onMount(() => {
    setTimeout(() => {
      window.location.href = DiscordInviteLink
    }, 10_000)
     
  })
</script>