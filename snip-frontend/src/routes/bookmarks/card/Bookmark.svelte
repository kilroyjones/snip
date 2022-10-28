<script lang="ts">
	import Fa from 'svelte-fa';
	import { faEdit, faTrashCan, faExternalLinkAlt } from '@fortawesome/free-solid-svg-icons';

	export let date: string;
	export let content: string;
	export let source: string;
	export let description: string;
	export let isHighlighted: boolean = false;

	let isExpanded: boolean = false;
	let textFragmentBookmark: string = '';
	let isLinkExpanded: boolean = false;
	// let fragment : string: source +

	function toggleExpand() {
		isExpanded = !isExpanded;
		console.log(isExpanded);
	}

	function toggleLinkExpand() {
		isLinkExpanded = !isLinkExpanded;
	}
</script>

<div id="bookmark-result" class={isHighlighted ? 'highlight' : ''}>
	<div id="bookmark-source">
		{#if isLinkExpanded}
			<Fa icon={faExternalLinkAlt} size="1x" fw scale={0.8} />
			<a href={source} target="_blank">{source}</a>
			<span class="link-expand" on:click={toggleLinkExpand}>[-]</span>
		{:else}
			<Fa icon={faExternalLinkAlt} size="1x" fw scale={0.8} />
			<a href={source} target="_blank">{new URL(source).origin}</a>
			<span class="link-expand" on:click={toggleLinkExpand}>[+]</span>
		{/if}
	</div>
	<div id="bookmark-description">{description}</div>
</div>

<style>
	#bookmark-result {
		display: flex;
		flex-direction: column;
		margin-top: 10px;
		margin-bottom: 20px;
		padding-right: 20px;
		max-width: 800px;
		font-family: 'Lato', Arial, sans-serif;
		font-weight: 400;
	}

	#bookmark-source {
		margin-bottom: 0px;
	}

	#bookmark-description {
		font-family: 'Lato', Arial, sans-serif;
		font-weight: 400;
		font-size: 18px;
		margin-top: 5px;
		margin-bottom: 8px;
	}

	.link-expand {
		font-family: 'Courier New', Courier, monospace;
		font-size: 12px;
		font-weight: 700;
		cursor: pointer;
		margin-left: 8px;
		color: #f4442e;
	}
</style>
