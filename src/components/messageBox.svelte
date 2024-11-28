<script lang="ts">
  import { postJSON, getJSON } from "../scripts/utils";'../scripts/utils.ts'

  let inputValue = "";
  let debounce = false;
  export let sendIcon = "";

  function validate() {
    const button = document.querySelector("#send");
    console.log(inputValue)
    if (inputValue.length <= 0) {
      button?.classList.add("invisible");
    } else {
      button?.classList.remove("invisible");
    }
  }

  function send() {
    var input: HTMLInputElement | null = document.querySelector("#sendPMInput");
    if (!input) return

    try {
      if (inputValue.length > 0 && !debounce) {
        console.log("Reaa")
        debounce = true;
        var _value = inputValue;
        var _xhr = "";
        inputValue = "";
        input.placeholder = "> Sending message..."
        postJSON("https://api.specifix.dev/mailbox", { content: _value }, 
          function (res) {
            input!.placeholder = "> Successfully sent!"
            debounce = false;
            setTimeout(() => {
              input!.placeholder = "> send me a message anonymously"
            }, 1500)
          },
          async function (xhr) {
            _xhr = `${xhr.status} ${await xhr.text()}`

            inputValue = "";
            input!.placeholder = `> Error: ${_xhr}`;
            setTimeout(() => {
              input!.placeholder = "> send me a message anonymously"
              debounce = false;
            }, 1500)
          }
        )
      } else {
        return
      }
    } catch (error) {
      debounce = true;
      inputValue = "";
      input.placeholder = `> ${error}`;
      setTimeout(() => {
        input!.placeholder = "> send me a message anonymously"
        debounce = false;
      }, 1500)
    } finally {
        return validate();
    }
  }
</script>

<div class="message-box-container">
  <input id="sendPMInput" bind:value={inputValue} on:input={validate} on:keydown={(event) => { event.key === "Enter" ? send() : null }} type="text" autocomplete="off" data-form-type="other" spellcheck="false" maxlength="512" autocapitalize="off" placeholder="> send me a message anonymously" aria-label="send me a message">
  <button id="send" class="invisible" aria-label="Send Message" on:click={send}>{@html sendIcon}</button>
</div>

  
<style lang="scss">
  .message-box-container {
    width: 85vw;
    max-width: 85%;
    height: 1.25em;
    padding: 0.6em;
    border-radius: var(--button-radius);
    background-color: var(--buttons);
    border: solid 1px var(--accent-2);
    color: var(--accent);
    font-family: var(--font-mono);
    margin: auto;
    margin-top: -0.5em;
    display: grid;
    align-items: center;
    grid-template-columns: 9fr 1fr;
    transition: box-shadow 100ms;

    &:hover {
      border: solid 1px var(--button-accent-hover);
      box-shadow: 0px 0px 8px var(--accent-3);
    }

    &:focus {
      border: solid 1px var(--button-accent-hover);
      outline: none;
    }
  }
  input {
    border: none;
    background: transparent;
    outline: none;
    font-family: inherit;
    color: inherit;
  }

  button {
    border: none;
    background: transparent;
    outline: none;
    font-family: inherit;
    font-size: 1.2em;
    color: inherit;
    margin-top: -0.25em;
    transition: transform 0.35s, filter 0.1s;

    &:hover {
      transform: scale(1.2);
      filter: brightness(1.1);
    }

    &:active {
      transform: scale(0.95);
    }
  }

  #send {
    transition: opacity 0.1s, transform 0.35s;
  }

  .invisible {
    opacity: 0%;
  }
</style>