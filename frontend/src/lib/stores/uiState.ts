const STORAGE_KEY = 'czkawka-ui-state';

export interface UiState {
	activeTool: string;
	selectedFile: string | null;
}

export function loadUiState(): UiState {
	if (typeof localStorage === 'undefined') {
		return { activeTool: 'duplicates', selectedFile: null };
	}
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) {
			return JSON.parse(raw);
		}
	} catch {
		// ignore
	}
	return { activeTool: 'duplicates', selectedFile: null };
}

export function saveUiState(state: UiState): void {
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
	}
}
