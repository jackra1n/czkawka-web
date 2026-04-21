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

<aside class="flex w-72 shrink-0 flex-col border-l border-border bg-surface">
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
