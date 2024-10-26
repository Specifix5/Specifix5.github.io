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

      getJSON("https://api.specifix.dev/api/ip", function(data) {
        document.querySelector("#ip")!.textContent = `${data.ip}${emoji}`;
        parseEmoji(document.querySelector("#ip")! as HTMLElement);
      });
    } catch (error) { console.log(error); }
  })
</script>

<span id="ip" class="blue"></span>