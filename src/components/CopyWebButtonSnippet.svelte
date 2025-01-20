<script lang="ts">
  import { onMount } from "svelte";

  onMount(() => {
    // thx vorlie !!!
    const copyButton = document.getElementById('copyButton');

    if (copyButton) {
      copyButton.addEventListener('click', async () => {
        const webButtonCode = `<a href="https://specifix.dev"><img src="https://specifix.dev/images/88x31.png" width="88" height="31" loading="lazy" alt="specifix.dev :3" /></a>`;
        try {
          if (navigator.clipboard) {
            await navigator.clipboard.writeText(webButtonCode);
          } else {
            // fallback to no clipboard
            const textArea = document.createElement("textarea");
            textArea.value = webButtonCode;
            textArea.style.position = 'fixed';
            textArea.style.top = '-1000px';
            document.body.appendChild(textArea);
            textArea.focus();
            textArea.select();
            document.execCommand("copy");
            document.body.removeChild(textArea);
          }

          alert('its copied!!!!!!');
        } catch (error) {
          console.error('Failed to copy webbutton: ', error);
          alert('it failed.. ' + JSON.stringify(error))
        }
      });
    }
  })
</script>

<button id="copyButton">
  <slot />
</button>

<style lang="scss">
  @use '../styles/vars.scss' as *;
  button {
    font-style: normal;
    text-decoration: underline;
    color: $accent-color;
    opacity: 100%;
    border: none;
    padding: 0px;
    margin: 0px;
    font-family: inherit;
    font-size: 1em;
    background: transparent;
  }
</style>