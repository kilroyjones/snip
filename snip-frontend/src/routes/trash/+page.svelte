<script lang="ts">
	import { onMount } from 'svelte';
	import { navigating } from '$app/stores';
	import { quotes, images, bookmarks, loadTrash, setSnippetTypes } from '$lib/loaders';
	import { setPagination } from '$lib/pagination';
	import Quotes from '../quotes/card/Quotes.svelte';
	import Images from '../images/card/Images.svelte';
	import Bookmarks from '../bookmarks/card/Bookmarks.svelte';

	async function setPageAttributes() {
		await setSnippetTypes(['quote', 'image', 'bookmark']);
		await setPagination(10, 1);
		await loadTrash();
	}

	// onMount(async () => {
	// 	setPageAttributes();
	// });

	$: if (navigating) setPageAttributes();
</script>

<Quotes quotes={$quotes} />
<Images images={$images} />
<Bookmarks bookmarks={$bookmarks} />
