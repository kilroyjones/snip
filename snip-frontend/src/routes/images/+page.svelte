<script lang="ts">
	import { onMount } from 'svelte';
	import { images, loadImages, setSnippetTypes } from '$lib/loaders';
	import { setPagination } from '$lib/pagination';
	import Images from './card/Images.svelte';
	import ImageFilters from './filters/ImageFilters.svelte';

	async function setPageAttributes() {
		console.log('Here');
		await setSnippetTypes(['image']);
		await setPagination(10, 1);
		await loadImages();
	}

	onMount(async () => {
		console.log('Image mount');
		setPageAttributes();
	});
</script>

<ImageFilters />
<Images images={$images} />
