// import { establishConnection } from '$lib/server_side_events/establishConnection';
import { loadQuotes, loadBookmarks, loadImages } from '$lib/loaders';
import { writable } from 'svelte/store';

export const establishSSEConnection = async function () {
	const source = new EventSource('http://localhost:8080/events');

	source.addEventListener(
		'message',
		function (event) {
			if (event) {
				let msg = JSON.parse(event.data);
				switch (msg.op) {
					case 'update':
						if (msg.data == 'quote') loadQuotes();
						if (msg.data == 'image') loadImages();
						if (msg.data == 'bookmark') loadBookmarks();
						break;
					case 'trash':
						break;
				}
			}
		},
		false
	);

	source.addEventListener(
		'open',
		function (e) {
			// Connection was opened.
			console.log('connected', e);
		},
		false
	);

	source.addEventListener(
		'close',
		function (e) {
			// Connection was opened.
			console.log('Close', e);
		},
		false
	);

	source.addEventListener('error', function (e) {}, false);
};
