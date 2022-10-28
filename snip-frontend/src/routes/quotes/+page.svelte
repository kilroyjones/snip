<script lang="ts">
  import { onMount } from "svelte";
  import { setPagination } from "$lib/pagination";
  import { loadQuotes, quotes } from "$lib/loaders";
  import QuoteFilters from "./filter/QuoteFilters.svelte";
  import Quotes from "./card/Quotes.svelte";

  async function setPageAttributes() {
    await setPagination(50, 1);
    await loadQuotes();
  }

  onMount(async () => {
    await setPageAttributes();
  });
</script>

<QuoteFilters />
<Quotes quotes={$quotes} />
