import { writable, get } from 'svelte/store';
import { getSnippets } from './server';
import { pageLimit, pageNumber } from './pagination';

export const snippetTypes: any = writable([]);
export const quotes: any = writable([]);
export const images: any = writable([]);
export const bookmarks: any = writable([]);

export async function setSnippetTypes(types: Array<string>) {
	snippetTypes.set(types);
}

export async function loadQuotes() {
	let res = await getSnippets('http://localhost:8080/get-quotes', get(pageLimit), get(pageNumber));
	// Check if res is ok
	quotes.update((_: any) => res);
}

export async function loadImages() {
	let res = await getSnippets('http://localhost:8080/get-images', get(pageLimit), get(pageNumber));
	// Check if res is ok
	images.update((_: any) => res);
}

export async function loadBookmarks() {
	let res = await getSnippets(
		'http://localhost:8080/get-bookmarks',
		get(pageLimit),
		get(pageNumber)
	);
	// Check if res is ok
	bookmarks.update((_: any) => res);
}

export async function loadTrash() {
	let res = await getSnippets(
		'http://localhost:8080/get-trash',
		get(snippetTypes),
		get(pageLimit),
		get(pageNumber)
	);
	// Check if res is ok
	parseSnippets(res);
}

async function parseSnippets(snippets: any) {
	//How does updating this affect things?
	// Look at reworking this. I think it's refreshing multiple times
	if (snippets) {
		snippets.forEach((snippet: any) => {
			switch (snippet.snippet_type) {
				case 'quote':
					quotes.update((_quotes: any) => [..._quotes, snippet]);
					break;
				case 'image':
					images.update((_images: any) => [..._images, snippet]);
					break;
				case 'bookmark':
					bookmarks.update((_bookmarks: any) => [..._bookmarks, snippet]);
					break;
			}
		});
	}
}
