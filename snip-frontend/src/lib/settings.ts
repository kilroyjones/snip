import { writable, get } from 'svelte/store';

export const showSource = writable(false);
export const toggleSourceVisibility = {
	subscribe: showSource,
	toggle: () => {
		showSource.update((value) => !value);
	}
};

export const showDescription = writable(false);
export const toggleDescriptionVisibility = {
	subscribe: showDescription,
	toggle: () => {
		showDescription.update((value) => !value);
	}
};

export const darkMode = writable(false);
