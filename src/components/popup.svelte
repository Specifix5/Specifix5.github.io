<script lang="ts">
  export let title: string = "";
  export let name: string = "";
  function closePopup() {
    let popup = document.querySelector("#popup_" + name)
    popup?.setAttribute("data-popup-visible", "false")
  }
</script>

<div id={"popup_" + name} class="popup" data-popup-visible="false">
  <div class="p-title">{@html title}</div>
  <div class="p-area">
    <slot />
  </div>
  <div class="p-footer"><button on:click={closePopup}>&lt; Back</button></div>
</div>

<style lang="scss">

  .popup {
    background-color: var(--buttons);
    width: 100%;
    max-width: 500px;
    left: 50%;
    top: 50%;
    height: 95vh;
    transform: translate(-50%, 50%) scale(0);
    transition: transform 100ms;
    border: solid 3px var(--accent-3);
    border-radius: var(--button-radius);

    position: fixed;

    display: grid;
    grid-template-rows: auto 1fr auto;
    grid-template-columns: 100%;
    row-gap: 10px;

    @media screen and (max-width: 485px) {
      height: 100%;
    }
  }

  .popup button {
    padding: 0.5em;
    border-radius: var(--button-radius);
    background-color: var(--buttons);
    border: solid 1px var(--accent-2);
    color: var(--accent);
    font-family: var(--font-mono);
    font-size: 1em;
    height: 100%;
    width: 100%;
    transition: transform 0.1s;

    &:active {
      transform: scale(0.95);
    }
  }

  .popup .p-title {
    background-color: var(--accent-3);
    height: 2rem;
    padding: 1rem;
    font-size: 25px;
    text-align: left;
  }

  .popup .p-area {
    padding: 0.8rem;
    padding-top: 0.2rem;
    text-wrap: pretty;
    height: 100%;
    overflow-y: scroll;
    text-align: left;
  }

  :global(.popup .p-area img) {
    border-radius: var(--button-radius);
    max-width: 100%;
  }

  :global(.p-area > ul) {
    padding-left: 0px;
  }

  :global(.p-area > .text-backdrop) {
    text-overflow: ellipsis;
    overflow: hidden;
    white-space: nowrap;
  }

  :global(.popup[data-popup-visible="true"]) {
    transform: translate(-50%, -50%) scale(1);
    display: grid;
  }

  .popup .p-footer {
    height: 1rem;
    padding: 0.35em;
    height: 2.5em;
    background-color: var(--accent-3);
  }
</style>