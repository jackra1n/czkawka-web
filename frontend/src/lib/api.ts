const API_BASE = '';

export interface ScanRequest {
	directories: string[];
	exclude_directories?: string[];
	min_file_size?: number;
}

export interface ScanResponse {
	id: string;
	status: string;
}

export interface DuplicateGroup {
	size: number;
	hash: string;
	files: string[];
}

export interface ScanResults {
	total_duplicate_groups: number;
	total_duplicate_files: number;
	wasted_space_bytes: number;
	scanning_time_ms: number;
	groups: DuplicateGroup[];
}

export interface ScanStatusResponse {
	id: string;
	status: string;
	results?: ScanResults;
	error?: string;
}

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
	}
};
