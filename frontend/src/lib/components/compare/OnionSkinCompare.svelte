<script lang="ts">
	import { formatBytes } from '$lib/utils';

	let {
		fileUrl,
		selectedFile,
		selectedFileSize,
		selectedFileDims,
		compareUrl,
		compareTarget,
		compareFileDims,
		onionOpacity
	}: {
		fileUrl: string;
		selectedFile: string;
		selectedFileSize: number;
		selectedFileDims: string;
		compareUrl: string;
		compareTarget: string;
		compareFileDims: string;
		onionOpacity: number;
	} = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<div class="mb-2 flex shrink-0 gap-2 px-2 pt-2">
		<div class="min-w-0 flex-1 truncate text-xs text-text-muted" title={selectedFile}>
			Selected: {selectedFile}
		</div>
		<div class="min-w-0 flex-1 truncate text-right text-xs text-text-muted" title={compareTarget}>
			Compare: {compareTarget}
		</div>
	</div>
	<div class="relative min-h-0 flex-1 bg-bg">
		<img src={compareUrl} alt="Compare" class="absolute inset-0 h-full w-full object-contain" />
		<img
			src={fileUrl}
			alt="Selected"
			class="absolute inset-0 h-full w-full object-contain"
			style="opacity: {onionOpacity}"
		/>
		<div class="absolute right-4 bottom-4 left-4 flex items-center gap-2">
			<span class="text-xs text-text-muted">Base</span>
			<input
				type="range"
				min="0"
				max="1"
				step="0.01"
				bind:value={onionOpacity}
				class="accent-primary h-1 flex-1 cursor-pointer"
			/>
			<span class="text-xs text-text-muted">Selected</span>
		</div>
	</div>
	<div class="mt-2 flex shrink-0 gap-2 px-2 pb-2">
		<div class="min-w-0 flex-1 truncate text-xs text-text-muted">
			{formatBytes(selectedFileSize)}{selectedFileDims ? ` • ${selectedFileDims}` : ''}
		</div>
		<div class="min-w-0 flex-1 truncate text-right text-xs text-text-muted">
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>
