<script lang="ts">
  import { onMount } from "svelte";
  import Source from "./Source.svelte";
  import Preview from "./Preview.svelte";
  import Options from "./Options.svelte";
  import Description from "./Description.svelte";

  export let id: number;
  export let searchId: number;
  export let date: string;
  export let quote: string;
  export let code_type: string;
  export let source: string;
  export let description: string;
  export let icon: string;
  export let updatePreview: any;

  async function select() {}
</script>

<div class="quote-container" on:click={() => updatePreview(searchId)}>
  <div class="quote-result">
    <div class="favicon-container">
      {#if icon != ""}
        <img class="favicon-img" src="http://localhost:8080/{icon}" alt="Favicon-{source}" />
      {:else}
        <img
          class="favicon-img"
          src="http://localhost:8080/files/images/default/quote.svg"
          alt="Favicon-{source}"
        />
      {/if}
    </div>
    <div class="quote-details">
      <div class="quote-description">
        {description}
      </div>
      <div class="quote-summary">
        {quote}
      </div>
    </div>
  </div>
</div>

<style>
  .favicon-container {
    /* background-color: white; */
    border-radius: 5px;
  }

  .quote-description {
    font-weight: 700;
  }
  .quote-summary {
    margin-top: 4px;
  }
  .quote-container {
    display: flex;
    flex-direction: row;
    cursor: pointer;
    padding-left: 10px;
  }

  .quote-details {
    display: flex;
    flex-direction: column;
  }

  .quote-result {
    display: flex;
    flex-direction: row;
    margin-top: 10px;
    margin-bottom: 20px;
    padding-right: 20px;
    width: 100vw;
    font-family: "Lato", Arial, sans-serif;
    font-weight: 400;
  }

  .favicon-img {
    width: 20px;
    padding-right: 20px;
    padding-top: 10px;
  }

  .quote-container:hover {
    background-color: lightgray;
  }
</style>
