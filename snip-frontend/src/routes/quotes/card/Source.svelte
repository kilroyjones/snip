<script lang="ts">
	import Fa from 'svelte-fa';
	import { faExternalLinkAlt } from '@fortawesome/free-solid-svg-icons';

	export let source: any;
	export let date: string;

	let isLinkExpanded: boolean = false;

	function toggleLinkExpand() {
		isLinkExpanded = !isLinkExpanded;
	}
</script>

<div id="quote-source">
	<div id="quote-link">
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
	<div id="quote-date">
		{date}
	</div>
</div>

<style>
	.link-expand {
		font-family: 'Courier New', Courier, monospace;
		font-size: 12px;
		font-weight: 700;
		cursor: pointer;
		margin-left: 8px;
		color: #f4442e;
	}
	#quote-source a,
	a:hover,
	a:active {
		color: #363821;
	}

	#quote-date {
		align-self: flex-end;
		font-size: 13px;
	}

	#quote-link {
		align-self: flex-start;
	}

	#quote-source {
		display: flex;
		justify-content: space-between;
	}

	#quote-source {
		font-size: 14px;
		margin-bottom: 0px;
	}
</style>
