<script lang="ts">
  import { onMount } from "svelte";

  let mikus = [
    `<img draggable="false" src="/images/miku_hello.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_love.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_plush.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_comfy.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_smirk.png" alt="m" class="emoji">`
  ];

  let html_string = "";

  onMount(() => {
    generateRandomMikus();
  });

  function generateRandomMikus() {
    html_string = "";
    for (let i = 0; i < 4; i++) {
      let miku = mikus[Math.floor(Math.random() * mikus.length)];
      html_string += miku;
    }
  }
</script>

{@html html_string}
