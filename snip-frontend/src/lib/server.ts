import { pageCount } from './pagination';

export async function getSnippets(url: string, limit: number, page: number, trash = false) {
	console.log(url);
	const response = await fetch(url, {
		method: 'POST',
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			limit: limit,
			offset: (page - 1) * limit,
			trash: trash
		})
	});
	if (response.status === 200) {
		let res = await response.json();
		console.log(res);
		pageCount.update((count: number) => res.pages);
		console.log(res.records);
		return res.records;
	} else {
		console.log('ERROR');
		console.log(response.statusText);
		return -1;
	}
}

export async function trashSnippet(id: number) {
	console.log(id);
	const response = await fetch('http://localhost:8080/trash-snippet', {
		method: 'POST',
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({
			id: id
		})
	});

	if (response.status === 200) {
		let res: any = await response.json();
		console.log('Success', res);
	} else {
		console.log('error');
	}
}
