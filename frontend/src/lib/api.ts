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
	// Same music options
	music_check_type?: string;
	// Broken files options
	broken_file_types?: string;
	// Bad extensions options
	include_files_without_extension?: boolean;
	// Bad names options
	bad_name_uppercase_extension?: boolean;
	bad_name_emoji?: boolean;
	bad_name_spaces?: boolean;
	bad_name_non_ascii?: boolean;
	bad_name_restricted_charset?: boolean;
	bad_name_allowed_chars?: string;
	bad_name_dedupe_non_alnum?: boolean;
}

export interface FixRequest {
	tool_id: string;
	files: string[];
	bad_name_uppercase_extension?: boolean;
	bad_name_emoji?: boolean;
	bad_name_spaces?: boolean;
	bad_name_non_ascii?: boolean;
	bad_name_restricted_charset?: boolean;
	bad_name_allowed_chars?: string;
	bad_name_dedupe_non_alnum?: boolean;
}

export interface FailedFix {
	path: string;
	error: string;
}

export interface FixResponse {
	fixed: string[];
	failed: FailedFix[];
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
	size?: number;
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

export interface ScanProgress {
	stage_label: string;
	current_stage_idx: number;
	max_stage_idx: number;
	entries_checked: number;
	entries_to_check: number;
	bytes_checked: number;
	bytes_to_check: number;
}

export interface ScanStatusResponse {
	id: string;
	status: string;
	progress?: ScanProgress;
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
	last_browser_directory?: string | null;
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
	default_directory?: string;
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

export interface SameMusicConfig {
	music_check_type: string;
}

export interface BrokenFilesConfig {
	broken_file_types: string;
}

export interface BadExtensionsConfig {
	include_files_without_extension: boolean;
}

export interface BadNamesConfig {
	bad_name_uppercase_extension: boolean;
	bad_name_emoji: boolean;
	bad_name_spaces: boolean;
	bad_name_non_ascii: boolean;
	bad_name_restricted_charset: boolean;
	bad_name_allowed_chars: string;
	bad_name_dedupe_non_alnum: boolean;
}

export type ToolConfig = Partial<
	BigFilesConfig &
		SimilarVideosConfig &
		SimilarImagesConfig &
		SameMusicConfig &
		BrokenFilesConfig &
		BadExtensionsConfig &
		BadNamesConfig
>;

async function fetchJson<T>(url: string, options?: RequestInit): Promise<T> {
	const res = await fetch(`${API_BASE}${url}`, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options?.headers,
		},
	});
	if (!res.ok) {
		const text = await res.text().catch(() => 'Unknown error');
		throw new Error(`HTTP ${res.status}: ${text}`);
	}
	return res.json();
}

async function fetchVoid(url: string, options?: RequestInit): Promise<void> {
	const res = await fetch(`${API_BASE}${url}`, {
		...options,
		headers: {
			'Content-Type': 'application/json',
			...options?.headers,
		},
	});
	if (!res.ok) {
		const text = await res.text().catch(() => 'Unknown error');
		throw new Error(`HTTP ${res.status}: ${text}`);
	}
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
			body: JSON.stringify(request),
		});
	},

	getScanStatus(id: string): Promise<ScanStatusResponse> {
		return fetchJson(`/api/scan/${id}`);
	},

	cancelScan(id: string): Promise<void> {
		return fetchVoid(`/api/scan/${id}/cancel`, {
			method: 'POST',
		});
	},

	getState(): Promise<AppState> {
		return fetchJson('/api/state');
	},

	updateDirectories(included: string[], excluded: string[], excluded_items: string): Promise<void> {
		return fetchVoid('/api/state/directories', {
			method: 'POST',
			body: JSON.stringify({ included, excluded, excluded_items }),
		});
	},

	getDefaults(): Promise<DefaultsResponse> {
		return fetchJson('/api/defaults');
	},

	getBrowserDirectory(): Promise<{ path: string }> {
		return fetchJson('/api/browser-directory');
	},

	updateToolState(toolId: string, checkedFiles: string[]): Promise<void> {
		return fetchVoid(`/api/state/tools/${toolId}`, {
			method: 'POST',
			body: JSON.stringify({ checked_files: checkedFiles }),
		});
	},

	deleteFiles(toolId: string, files: string[]): Promise<DeleteResponse> {
		return fetchJson('/api/delete', {
			method: 'POST',
			body: JSON.stringify({ tool_id: toolId, files }),
		});
	},

	fixFiles(request: FixRequest): Promise<FixResponse> {
		return fetchJson('/api/fix', {
			method: 'POST',
			body: JSON.stringify(request),
		});
	},
};
