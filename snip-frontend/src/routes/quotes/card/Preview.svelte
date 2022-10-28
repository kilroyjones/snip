<script lang="ts">
	import { HighlightAuto } from 'svelte-highlight';
	import github from 'svelte-highlight/styles/github';

	export let quote: string;
	export let code_type: string;

	let isQuoteExpanded: boolean = false;

	function toggleQuoteExpand() {
		isQuoteExpanded = !isQuoteExpanded;
	}
</script>

<svelte:head>
	{@html github}
</svelte:head>

<div id="quote-preview" on:dblclick={toggleQuoteExpand}>
	{#if code_type == ''}
		<div id="quote-formatted">
			{#if isQuoteExpanded}
				{@html quote}
			{:else if quote.length > 80}
				{@html quote.substring(0, 80)}...
			{:else}
				{@html quote}
			{/if}
		</div>
	{:else}
		<div id="quote-formatted">
			{#if isQuoteExpanded}
				<HighlightAuto code={quote} />
			{:else if quote.length > 80}
				<HighlightAuto code={quote.substring(0, 80)} />
			{:else}
				<HighlightAuto code={quote} />
			{/if}
		</div>
	{/if}
	<div id="quote-details">
		<div>
			{#if isQuoteExpanded}
				<span on:click={toggleQuoteExpand}>[-Details]</span>
			{:else}
				<span on:click={toggleQuoteExpand}>[+Details]</span>
			{/if}
		</div>
	</div>
</div>

<style>
	#quote-details {
		display: flex;
		font-family: 'Courier New', Courier, monospace;
		font-size: 12px;
		font-weight: 700;
		cursor: pointer;
		margin-top: 8px;
		color: #f4442e;
	}

	#quote-formatted {
		white-space: pre-wrap;

		/* line-clamp: 3; */
	}
</style>
