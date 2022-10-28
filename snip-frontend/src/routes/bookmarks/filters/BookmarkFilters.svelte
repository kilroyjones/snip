<script lang="ts">
	import Pagination from '$lib/components/search-filters/Pagination.svelte';
	import { loadBookmarks } from '$lib/loaders';
</script>

<Pagination loader={loadBookmarks} />
