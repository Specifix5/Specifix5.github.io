<script lang="ts">
  let mikus = [
    `<img draggable="false" src="/images/miku_hello.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_love.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_plush.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_comfy.png" alt="m" class="emoji">`,
    `<img draggable="false" src="/images/miku_smirk.png" alt="m" class="emoji">`
  ];

  // Reactive statement to regenerate `html_string`
  $: html_string = generateRandomMikus();

  function generateRandomMikus() {
    let html = "";
    for (let i = 0; i < 4; i++) {
      let miku = mikus[Math.floor(Math.random() * mikus.length)];
      html += miku;
    }
    return html;
  }
</script>

{@html html_string}
