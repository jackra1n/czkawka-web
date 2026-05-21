<script lang="ts">
	import { formatBytes } from '$lib/utils';
	import Tooltip from '../ui/Tooltip.svelte';

	let {
		fileUrl,
		selectedFile,
		selectedFileSize,
		selectedFileDims,
		compareUrl,
		compareTarget,
		compareFileDims,
		colorCodingEnabled,
		onMediaError,
	}: {
		fileUrl: string;
		selectedFile: string;
		selectedFileSize: number;
		selectedFileDims: string;
		compareUrl: string;
		compareTarget: string;
		compareFileDims: string;
		colorCodingEnabled: boolean;
		onMediaError: () => void;
	} = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<div class="mb-2 flex shrink-0 gap-2 px-2 pt-2">
		<Tooltip class="flex min-w-0 flex-1" content={selectedFile}>
			<div class="min-w-0 flex-1 truncate text-xs {colorCodingEnabled ? 'text-blue-400' : 'text-text-muted'}">
				Selected: {selectedFile}
			</div>
		</Tooltip>
		<Tooltip class="flex min-w-0 flex-1" content={compareTarget}>
			<div
				class="min-w-0 flex-1 truncate text-right text-xs {colorCodingEnabled ? 'text-amber-400' : 'text-text-muted'}"
			>
				Comparing: {compareTarget}
			</div>
		</Tooltip>
	</div>
	<div class="flex min-h-0 flex-1 bg-bg">
		<div class="h-full w-1/2 overflow-hidden">
			<img
				src={fileUrl}
				alt="Selected"
				class="h-full w-full object-contain object-right {colorCodingEnabled ? 'border-t-2 border-blue-500' : ''}"
				onerror={onMediaError}
			/>
		</div>
		<div class="h-full w-1/2 overflow-hidden">
			<img
				src={compareUrl}
				alt="Comparing"
				class="h-full w-full object-contain object-left {colorCodingEnabled ? 'border-t-2 border-amber-500' : ''}"
			/>
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
