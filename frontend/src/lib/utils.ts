export type PreviewType = 'image' | 'video' | 'audio' | 'text' | 'unknown';

export function getPreviewType(path: string): PreviewType {
	const ext = path.split('.').pop()?.toLowerCase() ?? '';
	if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'ico', 'tiff', 'avif'].includes(ext)) return 'image';
	if (['mp4', 'webm', 'mkv', 'avi', 'mov'].includes(ext)) return 'video';
	if (['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].includes(ext)) return 'audio';
	if (
		[
			'txt',
			'md',
			'rs',
			'py',
			'js',
			'ts',
			'svelte',
			'html',
			'css',
			'json',
			'yaml',
			'yml',
			'toml',
			'xml',
			'csv',
			'log',
			'sh',
			'c',
			'cpp',
			'h',
			'hpp',
			'java',
			'go',
			'rb',
			'php',
			'lua',
			'swift',
			'kt',
			'scala',
			'r',
			'pl',
			'sql',
		].includes(ext)
	)
		return 'text';
	return 'unknown';
}

export function formatBytes(bytes: number): string {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

export function formatDuration(ms: number): string {
	if (ms < 1000) return ms + ' ms';
	return (ms / 1000).toFixed(2) + ' s';
}

export function formatDate(timestamp: number | undefined): string {
	if (!timestamp) return '-';
	const d = new Date(timestamp);
	const pad = (n: number) => String(n).padStart(2, '0');
	return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}
