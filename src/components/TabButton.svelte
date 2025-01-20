<script lang="ts">
  import { openTab } from "../scripts/tabs";
  export let id: string = "";
  export let active: string = "";

  function openTabWrapper() {
    openTab(id, document.getElementById(`a-tab-${id}`));
  }
</script>

<button id={`a-tab-${id}`} onclick={openTabWrapper} class={active === "true" ? "tab-button-active" : ""}><slot /></button>