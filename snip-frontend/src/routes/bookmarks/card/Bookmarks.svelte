<script lang="ts">
	import Bookmark from './Bookmark.svelte';
	export let bookmarks: any;
</script>

{#each bookmarks as bookmark}
	<Bookmark
		date={bookmark.date}
		description={bookmark.description}
		content={bookmark.content}
		source={bookmark.source}
	/>
{/each}
