const STORAGE_KEY = 'czkawka-last-directory';

function getInitial(): string {
	if (typeof localStorage !== 'undefined') {
		return localStorage.getItem(STORAGE_KEY) || '~';
	}
	return '~';
}

let _lastDirectory = $state(getInitial());

export function getLastDirectory(): string {
	return _lastDirectory;
}

export function setLastDirectory(path: string): void {
	_lastDirectory = path;
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, path);
	}
}
