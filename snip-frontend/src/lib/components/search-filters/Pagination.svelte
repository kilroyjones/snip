<script lang="ts">
	import { pageCount, pageLimit, pageNumber, setPagination } from '$lib/pagination';
	export let loader: any;

	async function update(limit: number, page: number) {
		await setPagination(limit, page);
		await loader();
	}

	let resultsPerPage = [5, 10, 20, 30, 50, 100];
</script>

<div id="search-filters">
	<label for="per-page">Results per page: </label>
	<select bind:value={$pageLimit} on:change={() => update($pageLimit, $pageNumber)}>
		{#each resultsPerPage as perPage}
			<option value={perPage}>{perPage}</option>
		{/each}
	</select>
	{#each { length: $pageCount } as _, i}
		<button on:click={() => update($pageLimit, i + 1)}>{i + 1}</button>
	{/each}
</div>

<style>
	#search-filters {
		display: flex;
		flex-direction: row;
		margin-bottom: 10px;
	}
</style>
