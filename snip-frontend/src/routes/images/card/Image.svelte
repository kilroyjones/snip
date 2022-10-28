<script lang="ts">
	import { onMount } from 'svelte';
	import { showSource, showDescription } from '$lib/settings';

	export let id: number;
	export let date: string;
	export let content: string;
	export let divWidth: number;
	export let source: string;
	export let description: string;
	export let imagePath: string;
	export let thumbnailPath: string;

	// Taken from Sugar.js (MIT licensed)
	function truncate(str: string, limit: number) {
		var trimmable =
			'\u0009\u000A\u000B\u000C\u000D\u0020\u00A0\u1680\u180E\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200A\u202F\u205F\u2028\u2029\u3000\uFEFF';
		var reg = new RegExp('(?=[' + trimmable + '])');
		var words = str.split(reg);
		var count = 0;
		return words
			.filter(function (word) {
				count += word.length;
				return count <= limit;
			})
			.join('');
	}

	onMount(async () => {
		if ($showDescription) {
			description = truncate(description, 40);
		}
	});
</script>

<div
	id="image-result"
	style="flex: {100 / (divWidth / 200)}%; max-width: {100 / (divWidth / 200)}%"
>
	<a href="http://localhost:8080/{imagePath}" alt={description}>
		<div id="image">
			<img src="http://localhost:8080/{thumbnailPath}" alt={description} />
		</div>
	</a>

	{#if $showDescription}
		<div id="description">
			{description}
		</div>
	{/if}

	{#if $showSource}
		<div id="source">
			<a href={source}> Source </a>
		</div>
	{/if}
</div>

<style>
	#image-result {
		display: flex;
		flex-direction: column;
		padding-right: 16px;
		padding-bottom: 20px;
	}

	#description {
		font-size: 14px;
		white-space: nowrap;
		overflow: hidden;
	}

	#image > img {
		object-fit: cover;
		width: 100%;
		height: 130px;
		max-width: 200px;
		border-radius: 2px;
	}
</style>
