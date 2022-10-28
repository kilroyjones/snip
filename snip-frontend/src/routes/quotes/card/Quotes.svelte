<script lang="ts">
  import Quote from "./Quote.svelte";
  export let quotes: any;
  let preview: string;

  function trimQuote(quote: string) {
    return quote.split("\n")[0].substring(0, 50);
  }

  function updatePreview(idx: number) {
    preview = quotes[idx].quote;
  }
</script>

<div class="quotes-container">
  <div class="quotes-search-results">
    {#if quotes}
      {#each quotes as quote, idx}
        <div class="quote">
          <Quote
            id={quote.id}
            searchId={idx}
            date={quote.date}
            description={quote.description}
            quote={trimQuote(quote.quote)}
            code_type={quote.code_type}
            source={quote.source}
            icon={quote.icon_path}
            {updatePreview}
          />
        </div>
      {/each}
    {/if}
  </div>
  <div class="quote-preview">{@html preview}</div>
</div>

<style>
  .quotes-container {
    display: flex;
    flex-direction: row;
    min-height: 0;
  }

  .quotes-search-results {
    flex: 1;
    min-height: 0;
    min-width: 400px;
    max-width: 600px;
    overflow: auto;
  }

  .quote {
    width: 100%;
  }

  .quote-preview {
    width: 100%;
  }
</style>
