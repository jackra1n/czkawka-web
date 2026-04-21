import { writable } from 'svelte/store';

const STORAGE_KEY = 'czkawka-last-directory';

function createLastDirectoryStore() {
	const initial = typeof localStorage !== 'undefined'
		? localStorage.getItem(STORAGE_KEY) || '~'
		: '~';

	const { subscribe, set } = writable(initial);

	return {
		subscribe,
		set: (path: string) => {
			if (typeof localStorage !== 'undefined') {
				localStorage.setItem(STORAGE_KEY, path);
			}
			set(path);
		}
	};
}

export const lastDirectory = createLastDirectoryStore();
