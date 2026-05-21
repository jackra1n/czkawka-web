const STORAGE_KEY = 'czkawka-ui-state';

export interface UiState {
	activeTool: string;
	selectedFile: string | null;
	sidebarCollapsed: boolean;
}

export function loadUiState(): UiState {
	if (typeof localStorage === 'undefined') {
		return { activeTool: 'duplicates', selectedFile: null, sidebarCollapsed: false };
	}
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) {
			const parsed = JSON.parse(raw);
			return {
				activeTool: parsed.activeTool ?? 'duplicates',
				selectedFile: parsed.selectedFile ?? null,
				sidebarCollapsed: parsed.sidebarCollapsed ?? false,
			};
		}
	} catch {
		// ignore
	}
	return { activeTool: 'duplicates', selectedFile: null, sidebarCollapsed: false };
}

export function saveUiState(state: UiState): void {
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
	}
}
