const API_BASE = '';

export interface ScanRequest {
	directories: string[];
	exclude_directories?: string[];
	excluded_items?: string;
	min_file_size?: number;
	tool_id?: string;
	// Big files options
	number_of_files?: number;
	search_mode?: string;
	// Similar videos options
	tolerance?: number;
	vid_hash_duration?: number;
	crop_detect?: string;
	// Similar images options
	hash_alg?: string;
	hash_size?: number;
	resize_filter?: string;
	similarity?: number;
}

export interface ScanResponse {
	id: string;
	status: string;
}

export interface ScannedFile {
	path: string;
	modified_date?: number;
	dimensions?: string;
	similarity?: string;
}

export interface FileGroup {
	size: number;
	hash: string;
	files: ScannedFile[];
}

export interface ScanResults {
	total_groups: number;
	total_items: number;
	wasted_bytes: number;
	scanning_time_ms: number;
	groups: FileGroup[];
}

export interface ScanStatusResponse {
	id: string;
	status: string;
	results?: ScanResults;
	error?: string;
}

export interface AppState {
	directories: {
		included: string[];
		excluded: string[];
		excluded_items: string;
	};
	tools: Record<string, ToolState>;
}

export interface ToolState {
	status: string;
	scan_id?: string;
	error?: string;
	results?: ScanResults;
	checked_files?: string[];
}

export interface DefaultsResponse {
	excluded_directories: string[];
	excluded_items: string;
}

export interface FailedDeletion {
	path: string;
	error: string;
}

export interface DeleteResponse {
	deleted: string[];
	failed: FailedDeletion[];
}

export interface BigFilesConfig {
	number_of_files: number;
	search_mode: string;
}

export interface SimilarVideosConfig {
	tolerance: number;
	vid_hash_duration: number;
	crop_detect: string;
}

export interface SimilarImagesConfig {
	hash_alg: string;
	hash_size: number;
	resize_filter: string;
	similarity: number;
}

export type ToolConfig = Partial<BigFilesConfig & SimilarVideosConfig & SimilarImagesConfig>;

async function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
	const res = await fetch(`${API_BASE}${url}`, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options?.headers
		}
	});
	if (!res.ok) {
		const text = await res.text().catch(() => 'Unknown error');
		throw new Error(`HTTP ${res.status}: ${text}`);
	}
	return res.json();
}

export function getFileUrl(path: string): string {
	return `${API_BASE}/api/file?path=${encodeURIComponent(path)}`;
}

export async function fetchFileText(path: string, signal?: AbortSignal): Promise<string> {
	const res = await fetch(getFileUrl(path), { signal });
	if (!res.ok) {
		const text = await res.text().catch(() => 'Unknown error');
		throw new Error(`HTTP ${res.status}: ${text}`);
	}
	const text = await res.text();
	return text.length > 3000 ? text.slice(0, 3000) + '\n\n... (truncated)' : text;
}

export const api = {
	health(): Promise<string> {
		return fetch(`${API_BASE}/api/health`).then((r) => r.text());
	},

	startScan(request: ScanRequest): Promise<ScanResponse> {
		return fetchJson('/api/scan', {
			method: 'POST',
			body: JSON.stringify(request)
		});
	},

	getScanStatus(id: string): Promise<ScanStatusResponse> {
		return fetchJson(`/api/scan/${id}`);
	},

	getState(): Promise<AppState> {
		return fetchJson('/api/state');
	},

	updateDirectories(included: string[], excluded: string[], excluded_items: string): Promise<void> {
		return fetchJson('/api/state/directories', {
			method: 'POST',
			body: JSON.stringify({ included, excluded, excluded_items })
		});
	},

	getDefaults(): Promise<DefaultsResponse> {
		return fetchJson('/api/defaults');
	},

	updateToolState(toolId: string, checkedFiles: string[]): Promise<void> {
		return fetchJson(`/api/state/tools/${toolId}`, {
			method: 'POST',
			body: JSON.stringify({ checked_files: checkedFiles })
		});
	},

	deleteFiles(toolId: string, files: string[]): Promise<DeleteResponse> {
		return fetchJson('/api/delete', {
			method: 'POST',
			body: JSON.stringify({ tool_id: toolId, files })
		});
	}
};
