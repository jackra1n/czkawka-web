<script lang="ts">
	import { X, File } from 'lucide-svelte';
	import { formatBytes } from '$lib/utils';

	let {
		selectedFile,
		selectedFileSize,
		onClose
	}: {
		selectedFile: string;
		selectedFileSize: number;
		onClose: () => void;
	} = $props();
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
	<div class="flex flex-1 flex-col items-center gap-6 p-6">
		<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
			<File class="h-12 w-12 text-text-muted opacity-40" />
		</div>
		<div class="w-full space-y-4">
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
