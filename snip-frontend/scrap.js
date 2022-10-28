// <style>
// 	#search-container {
// 		padding: 20px;
// 		width: 100%;
// 		-webkit-box-sizing: border-box;
// 		-moz-box-sizing: border-box;
// 		box-sizing: border-box;
// 	}

// 	#search {
// 		border-radius: 5px;
// 		border: 1px solid lightgray;
// 		font-size: 18px;
// 		max-width: 100%;
// 		width: 100%;
// 		padding: 10px;
// 		-webkit-box-sizing: border-box;
// 		-moz-box-sizing: border-box;
// 		box-sizing: border-box;
// 	}

// 	.search-results {
// 		display: flex;
// 		flex-direction: column;
// 	}

// 	.search-result {
// 		border: 1px solid lightgray;
// 		border-radius: 5px;
// 		margin: 8px 0px 8px 0px;
// 		padding: 6px;
// 		-webkit-box-sizing: border-box;
// 		-moz-box-sizing: border-box;
// 		box-sizing: border-box;
// 	}

// 	.source {
// 		font-size: 18px;
// 		font-weight: 700;
// 	}

// 	.date {
// 		font-size: 14px;
// 	}

// 	.content {
// 		font-size: 16px;
// 	}
// </style>
// 		<input id="search-box" placeholder="Search for {$page.routeId}" bind:value={searchTerm} />

// 		<button id="search-submit" on:click={search}>Search</button>
// 	#search-bar {
// 		display: flex;
// 		flex-direction: row;
// 		min-height: 100px;
// 		height: 100px;
// 		width: 100%;
// 		align-items: center;
// 	}

// 	#search-box {
// 		height: 30px;
// 		align-self: center;
// 		flex-grow: 3;
// 		max-width: 700px;
// 		margin-left: 40px;
// 	}

// 	#logo {
// 		width: 100px;
// 		padding-left: 10px;
// 	}

// 	#search-properties {
// 		flex-grow: 1;
// 	}
// 	#search-submit {
// 		height: 30px;
// 	}
// <script lang="ts">
// 	import { showSource, showDescription } from '../resultsConfig.js';

// 	export let toggleSidebar: any;

// 	function toggle(configElement: any) {
// 		configElement = !configElement;
// 	}
// </script>

// <div id="sidebar">
// 	<div class="menu-item">
// 		<button on:click={toggleSidebar}>Test</button>
// 	</div>

// 	<div class="menu-item">
// 		<div class="menu-item-icon">
// 			<img src="images/quotes.svg" width="30px" alt="svg" />
// 		</div>
// 		<div class="menu-item-text">
// 			<a sveltekit:reload href="quotes">Quotes</a>
// 		</div>
// 	</div>

// 	<div class="menu-item">
// 		<div class="menu-item-icon">
// 			<img src="images/images.svg" width="30px" alt="svg" />
// 		</div>
// 		<div class="menu-item-text">
// 			<a sveltekit:reload href="images">Images</a>
// 		</div>
// 	</div>
// 	<label>
// 		<input type="checkbox" id="source" name="show-source" bind:checked={$showDescription} />
// 		Description
// 	</label>
// 	<label>
// 		<input type="checkbox" id="source" name="show-source" bind:checked={$showSource} />
// 		Source
// 	</label>
// </div>

// <style>
// 	#sidebar {
// 		display: flex;
// 		flex-direction: column;
// 		max-width: 30%;
// 		padding-top: 30px;
// 	}
// 	hr {
// 		border: 1px solid black;
// 		width: 100%;
// 	}

// 	.menu-item {
// 		display: flex;
// 		flex-direction: row;
// 		align-items: center;
// 	}

// 	.menu-item-icon {
// 		margin-right: 6px;
// 	}

// 	.menu-item-text {
// 		font-size: 18px;
// 	}
// </style>
// 		<button on:click={() => (isExpanded = !isExpanded)}>
// 			{#if isExpanded}<SidebarMenuToggle {isExpanded} icon={faBars} />{:else}{/if}
// 		</button>

// onMount(async () => {
// 	// Needs fixing to cover all cases for the fragment.
// 	// Also, there is some difficult when the text is really long, so trimming the start
// 	// and end would be preferable.
// 	let lines = (content.trim().match(/\r\n|\n|\r/g) || '').length + 1;
// 	if (lines > 1) {
// 		let splitContent = content.split(/\r?\n/);
// 		textFragmentLink =
// 			source + '#:~:text=' + splitContent[0] + ',' + splitContent[splitContent.length];
// 	} else {
// 		textFragmentLink = source + '/#:~:text=' + content;
// 	}
// });
// let textFragmentLink: string = '';

// let fragment : string: source +
// <!-- |<a href={textFragmentLink} alt="text fragment">Fragment</a> -->

// // server side

// //
// switch (snippet.snippet_type) {
// 	case 'quote':
// 		quotes.update((qtes: any) => [...qtes, snippet]);
// 		break;
// 	case 'image':
// 		images.update((imgs: any) => [...imgs, snippet]);
// 		break;
// 	case 'bookmark':
// 		bookmarks.update((bkmks: any) => [...bkmks, snippet]);
// 		break;
// }

// Probably should move this server side
async function parseSnippets(snippets: any) {
	snippets.forEach(async function (snippet: any) {
		let dateInMilliseconds: number = Date.parse(snippet.date);
		snippet.date = new Date(dateInMilliseconds).toLocaleString();
	});
	return snippets;
}

		<Source {source} {date} />
		<Description {description} {source} />
		<Preview {quote} {code_type} />