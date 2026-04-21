<script lang="ts">
	import { X, File, Loader2 } from 'lucide-svelte';
	import { formatBytes } from '$lib/utils';
	import { getFileUrl, fetchFileText } from '$lib/api';

	let {
		selectedFile,
		selectedFileSize,
		onClose
	}: {
		selectedFile: string;
		selectedFileSize: number;
		onClose: () => void;
	} = $props();

	type PreviewType = 'image' | 'video' | 'audio' | 'text' | 'unknown';

	function getPreviewType(path: string): PreviewType {
		const ext = path.split('.').pop()?.toLowerCase() ?? '';
		if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'ico', 'tiff', 'avif'].includes(ext)) return 'image';
		if (['mp4', 'webm', 'mkv', 'avi', 'mov'].includes(ext)) return 'video';
		if (['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].includes(ext)) return 'audio';
		if (['txt', 'md', 'rs', 'py', 'js', 'ts', 'svelte', 'html', 'css', 'json', 'yaml', 'yml', 'toml', 'xml', 'csv', 'log', 'sh', 'c', 'cpp', 'h', 'hpp', 'java', 'go', 'rb', 'php', 'lua', 'swift', 'kt', 'scala', 'r', 'pl', 'sql'].includes(ext)) return 'text';
		return 'unknown';
	}

	let textContent = $state('');
	let textLoading = $state(false);
	let textError = $state('');
	let mediaError = $state(false);

	let previewType = $derived(getPreviewType(selectedFile));
	let fileUrl = $derived(getFileUrl(selectedFile));

	const MIN_WIDTH = 200;
	const MAX_WIDTH_FRACTION = 0.6;
	const STORAGE_KEY = 'filePreviewWidth';

	let panelWidth = $state(288);

	$effect(() => {
		if (typeof window === 'undefined') return;
		const saved = localStorage.getItem(STORAGE_KEY);
		if (saved) {
			const parsed = parseInt(saved, 10);
			if (!isNaN(parsed)) {
				panelWidth = Math.max(MIN_WIDTH, Math.min(window.innerWidth * MAX_WIDTH_FRACTION, parsed));
			}
		}
	});

	function startResize(e: MouseEvent) {
		e.preventDefault();
		const startX = e.clientX;
		const startWidth = panelWidth;
		const body = document.body;
		const originalUserSelect = body.style.userSelect;
		body.style.userSelect = 'none';

		function onMouseMove(e: MouseEvent) {
			const delta = startX - e.clientX;
			const maxWidth = window.innerWidth * MAX_WIDTH_FRACTION;
			panelWidth = Math.max(MIN_WIDTH, Math.min(maxWidth, startWidth + delta));
		}

		function onMouseUp() {
			window.removeEventListener('mousemove', onMouseMove);
			window.removeEventListener('mouseup', onMouseUp);
			body.style.userSelect = originalUserSelect;
			localStorage.setItem(STORAGE_KEY, String(panelWidth));
		}

		window.addEventListener('mousemove', onMouseMove);
		window.addEventListener('mouseup', onMouseUp);
	}

	$effect(() => {
		const file = selectedFile;
		const type = previewType;
		textContent = '';
		textError = '';
		mediaError = false;

		if (type !== 'text') return;

		textLoading = true;
		const controller = new AbortController();

		fetchFileText(file, controller.signal)
			.then((content) => {
				textContent = content;
			})
			.catch((err) => {
				if (err.name !== 'AbortError') {
					textError = err.message ?? 'Failed to load file';
				}
			})
			.finally(() => {
				textLoading = false;
			});

		return () => {
			controller.abort();
		};
	});

	function handleMediaError() {
		mediaError = true;
	}
</script>

<aside class="relative flex shrink-0 flex-col border-l border-border bg-surface" style:width="{panelWidth}px">
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<div
		class="absolute left-0 top-0 bottom-0 w-4 cursor-col-resize z-20 flex items-center justify-center group"
		onmousedown={startResize}
		role="separator"
		aria-label="Resize preview panel"
		aria-orientation="vertical"
	>
		<div class="h-10 w-1.5 rounded-full bg-border flex flex-col items-center justify-center gap-0.5 opacity-60 group-hover:opacity-100 transition-opacity">
			<div class="w-px h-1 bg-text-muted rounded-full"></div>
			<div class="w-px h-1 bg-text-muted rounded-full"></div>
			<div class="w-px h-1 bg-text-muted rounded-full"></div>
		</div>
	</div>

	<div class="flex items-center justify-between border-b border-border px-4 py-3">
		<span class="text-sm font-medium">Preview</span>
		<button
			onclick={onClose}
			class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
		>
			<X class="h-4 w-4" />
		</button>
	</div>

	<div class="flex flex-1 flex-col min-h-0">
		{#if previewType === 'image'}
			{#if mediaError}
				<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
					<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
						<File class="h-12 w-12 text-text-muted opacity-40" />
					</div>
					<p class="text-xs text-text-muted">Failed to load image</p>
				</div>
			{:else}
				<div class="flex flex-1 items-center justify-center p-4 bg-bg">
					<img
						src={fileUrl}
						alt="Preview"
						class="max-h-full max-w-full object-contain rounded-md"
						onerror={handleMediaError}
					/>
				</div>
			{/if}
		{:else if previewType === 'video'}
			{#if mediaError}
				<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
					<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
						<File class="h-12 w-12 text-text-muted opacity-40" />
					</div>
					<p class="text-xs text-text-muted">Failed to load video</p>
				</div>
			{:else}
				<div class="flex flex-1 items-center justify-center p-4 bg-bg">
					<!-- svelte-ignore a11y_media_has_caption -->
					<video
						src={fileUrl}
						controls
						class="max-h-full max-w-full rounded-md"
						onerror={handleMediaError}
					></video>
				</div>
			{/if}
		{:else if previewType === 'audio'}
			{#if mediaError}
				<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
					<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
						<File class="h-12 w-12 text-text-muted opacity-40" />
					</div>
					<p class="text-xs text-text-muted">Failed to load audio</p>
				</div>
			{:else}
				<div class="flex flex-1 flex-col items-center justify-center gap-4 p-6 bg-bg">
					<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border">
						<File class="h-12 w-12 text-text-muted opacity-40" />
					</div>
					<audio
						src={fileUrl}
						controls
						class="w-full"
						onerror={handleMediaError}
					></audio>
				</div>
			{/if}
		{:else if previewType === 'text'}
			{#if textLoading}
				<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
					<Loader2 class="h-6 w-6 animate-spin text-text-muted" />
					<p class="text-xs text-text-muted">Loading text...</p>
				</div>
			{:else if textError}
				<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
					<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
						<File class="h-12 w-12 text-text-muted opacity-40" />
					</div>
					<p class="text-xs text-danger">{textError}</p>
				</div>
			{:else}
				<div class="flex-1 overflow-auto p-4 bg-bg">
					<pre class="text-xs font-mono text-text whitespace-pre-wrap break-all">{textContent}</pre>
				</div>
			{/if}
		{:else}
			<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
				<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
					<File class="h-12 w-12 text-text-muted opacity-40" />
				</div>
				<p class="text-xs text-text-muted">No preview available</p>
			</div>
		{/if}

		<div class="shrink-0 border-t border-border p-4 space-y-3">
			<div>
				<p class="mb-1 text-xs font-medium text-text-muted">Path</p>
				<p class="break-all text-xs font-mono leading-relaxed text-text">{selectedFile}</p>
			</div>
			<div>
				<p class="mb-1 text-xs font-medium text-text-muted">Size</p>
				<p class="text-sm text-text">{formatBytes(selectedFileSize)}</p>
			</div>
		</div>
	</div>
</aside>
