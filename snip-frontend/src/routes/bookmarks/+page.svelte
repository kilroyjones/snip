<script lang="ts">
	import { onMount } from 'svelte';
	import { bookmarks, loadBookmarks, setSnippetTypes } from '$lib/loaders';
	import { setPagination } from '$lib/pagination';
	import Bookmarks from '../bookmarks/card/Bookmarks.svelte';
	import BookmarkFilters from './filters/BookmarkFilters.svelte';

	async function setPageAttributes() {
		await setSnippetTypes(['bookmark']);
		await setPagination(10, 1);
		await loadBookmarks();
	}

	onMount(async () => {
		setPageAttributes();
	});
</script>

<BookmarkFilters />
<Bookmarks bookmarks={$bookmarks} />
