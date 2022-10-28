<script lang="ts">
	// import { paginationLimit, paginationPage, paginationCount } from '$lib/store';
	// export let fn;

	// let resultsPerPageSelected = 10;
	// let resultsPerPage = [5, 10, 20, 30, 50, 100];

	// async function onResultsPerPageChange() {
	// 	paginationLimit.update((res: any) => resultsPerPageSelected);
	// 	console.log($paginationLimit);
	// }

	// async function jumpToPage(page: number) {
	// 	paginationPage.update((res: any) => page);
	// }
</script>

<!-- <div id="search-filters">
	<label for="per-page">Results per page: </label>
	<select bind:value={resultsPerPageSelected} on:change={onResultsPerPageChange}>
		{#each resultsPerPage as perPage}
			<option value={perPage}>{perPage}</option>
		{/each}
	</select>
	{#each { length: $paginationCount } as _, i}
		<button on:click={() => jumpToPage(i + 1)}>{i + 1}</button>
	{/each}
</div>

<style>
	#search-filters {
		display: flex;
		flex-direction: row;
		margin-bottom: 10px;
	}
</style> -->
