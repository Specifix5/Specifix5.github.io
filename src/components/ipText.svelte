<script lang="ts">
  import { onMount } from "svelte";
  import { getJSON, parseEmoji } from "../scripts/utils";
  onMount(async () => {
    let ipElement = document.getElementById("ip-text");
    if (!ipElement) return;
    let ip = (await getJSON("https://api.specifix.dev/ip"))["ip"];
    let hour = Number(new Date().toLocaleString('en-US', {hour12: false}).split(", ")[1].split(":")[0]);
    let emoji = "☀️";

    if (hour < 18) {
      if (hour < 6) {
        emoji = `🌙`;
      }
    } else {
      emoji = `🌙`;
    }

    ipElement.innerText = ip + emoji;
    parseEmoji(ipElement)
  })
</script>

<span class="blue" id="ip-text"></span>