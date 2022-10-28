<script>
  import browser from "webextension-polyfill";
  let image = "";

  function change() {
    // browser.storage.local.set({ background: image });
    console.log("Her");
  }
</script>

<main>
  Image URL: <input type="text" bind:value={image} />
  <button on:click={change}>Change</button>
</main>

<style>
  main {
    width: 100vh;
    height: 100vh;
  }
</style>
