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
		onionOpacity,
		colorCodingEnabled
	}: {
		fileUrl: string;
		selectedFile: string;
		selectedFileSize: number;
		selectedFileDims: string;
		compareUrl: string;
		compareTarget: string;
		compareFileDims: string;
		onionOpacity: number;
		colorCodingEnabled: boolean;
	} = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<div class="mb-2 flex shrink-0 gap-2 px-2 pt-2">
		<div class="min-w-0 flex-1 truncate text-xs text-text-muted" title={selectedFile}>
			Selected: {selectedFile}
		</div>
		<div class="min-w-0 flex-1 truncate text-right text-xs text-text-muted" title={compareTarget}>
			Comparing: {compareTarget}
		</div>
	</div>
	<div class="relative min-h-0 flex-1 bg-bg">
		<img src={fileUrl} alt="Selected" class="absolute inset-0 h-full w-full object-contain" />
		<img
			src={compareUrl}
			alt="Comparing"
			class="absolute inset-0 h-full w-full object-contain"
			style="opacity: {onionOpacity}"
		/>
		<div class="absolute inset-x-0 bottom-4 flex justify-center px-4">
			<div class="flex w-full max-w-sm items-center gap-2 rounded-lg bg-black/40 px-3 py-2">
				<span class="text-xs {colorCodingEnabled ? 'text-blue-400' : 'text-text'}">Selected</span>
				<input
					type="range"
					min="0"
					max="1"
					step="0.01"
					bind:value={onionOpacity}
					class="accent-primary h-1 flex-1 cursor-pointer"
				/>
				<span class="text-xs {colorCodingEnabled ? 'text-amber-400' : 'text-text'}">Comparing</span>
			</div>
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
