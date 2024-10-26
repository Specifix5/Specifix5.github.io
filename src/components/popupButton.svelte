<script lang="ts">
  import { openPopup, parseEmoji } from "../scripts/utils";

  export let popupName: string = "";
  export let buttonText: string = "";

  function onClick() {
    openPopup(popupName)
  }
</script>

<button class="fancyButton" on:click={onClick}>{@html parseEmoji(buttonText)}</button>

<style>
  .fancyButton {
    height: 100%;
    width: 100%;
  }
</style>

