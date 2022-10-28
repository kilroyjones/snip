<script lang="ts">
	import Pagination from '$lib/components/search-filters/Pagination.svelte';
	import { loadImages } from '$lib/loaders';
</script>

<Pagination loader={loadImages} />
