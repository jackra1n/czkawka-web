const STORAGE_KEY = 'czkawka-ui-state';

export interface UiState {
	activeTool: string;
	selectedFile: string | null;
	sidebarCollapsed: boolean;
	showSettings: boolean;
	hideHardLinks: boolean;
}

export function loadUiState(): UiState {
	if (typeof localStorage === 'undefined') {
		return { activeTool: 'duplicates', selectedFile: null, sidebarCollapsed: false, showSettings: false, hideHardLinks: true };
	}
	try {
		const raw = localStorage.getItem(STORAGE_KEY);
		if (raw) {
			const parsed = JSON.parse(raw);
			return {
				activeTool: parsed.activeTool ?? 'duplicates',
				selectedFile: parsed.selectedFile ?? null,
				sidebarCollapsed: parsed.sidebarCollapsed ?? false,
				showSettings: parsed.showSettings ?? false,
				hideHardLinks: parsed.hideHardLinks ?? true,
			};
		}
	} catch {
		// ignore
	}
	return { activeTool: 'duplicates', selectedFile: null, sidebarCollapsed: false, showSettings: false, hideHardLinks: true };
}

export function saveUiState(state: UiState): void {
	if (typeof localStorage !== 'undefined') {
		localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
	}
}
