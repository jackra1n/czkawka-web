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

<div class="flex flex-1 min-h-0 flex-col">
	<div class="flex shrink-0 gap-2 px-2 pt-2 mb-2">
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted" title={selectedFile}>Selected: {selectedFile}</div>
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted text-right" title={compareTarget}>Compare: {compareTarget}</div>
	</div>
	<div class="relative flex-1 min-h-0 bg-bg">
		<img src={compareUrl} alt="Compare" class="absolute inset-0 w-full h-full object-contain" />
		<img src={fileUrl} alt="Selected" class="absolute inset-0 w-full h-full object-contain" style="opacity: {onionOpacity}" />
		<div class="absolute bottom-4 left-4 right-4 flex items-center gap-2">
			<span class="text-xs text-text-muted">Base</span>
			<input
				type="range"
				min="0"
				max="1"
				step="0.01"
				bind:value={onionOpacity}
				class="flex-1 h-1 accent-primary cursor-pointer"
			/>
			<span class="text-xs text-text-muted">Selected</span>
		</div>
	</div>
	<div class="flex shrink-0 gap-2 px-2 pb-2 mt-2">
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted">
			{formatBytes(selectedFileSize)}{selectedFileDims ? ` • ${selectedFileDims}` : ''}
		</div>
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted text-right">
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>