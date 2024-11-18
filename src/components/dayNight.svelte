<script lang="ts">
  import { getJSON, parseEmoji } from '../scripts/utils'
  import { onMount } from 'svelte';

  onMount(() => {
    try {
      let hour = Number(new Date().toLocaleString('en-US', {hour12: false}).split(", ")[1].split(":")[0]);
      let emoji = "☀️";
      let daynight = "day";

      if (hour < 18) {
        if (hour < 6) {
          emoji = `🌙`;
          daynight = "evening";
        }
      } else {
        emoji = `🌙`;
        daynight = "evening";
      }

      document.querySelector("#daynight")!.textContent = `${daynight}${emoji}`;
      parseEmoji(document.querySelector("#daynight")! as HTMLElement);
    } catch (error) { console.log(error); }
  })
</script>

<span id="daynight" class="blue">day</span>