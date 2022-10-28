<script lang="ts">
	import Pagination from '$lib/components/search-filters/Pagination.svelte';
	import { loadQuotes } from '$lib/loaders';
</script>

<Pagination loader={loadQuotes} />
