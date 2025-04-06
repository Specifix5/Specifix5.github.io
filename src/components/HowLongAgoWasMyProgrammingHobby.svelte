<script lang="ts">
  import { onMount } from 'svelte'; 
  const howLongAgoWasIt = 1520553600000;
  onMount(() => {
    setInterval(() => {
    let relativeTime = Date.now() - howLongAgoWasIt;
    document.getElementById("long-ago")!.innerText = `${relativeTime / 1_000 / 60 / 60 / 24 / 30 / 12} years`; // totally a random day in march
  }, 1_000)
  })
</script>

<span class="blue" id="long-ago"></span>