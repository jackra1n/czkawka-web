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
		colorCodingEnabled,
		onMediaError
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

<div class="flex min-h-0 flex-1 items-center justify-center gap-2 overflow-hidden bg-bg p-2">
	<div class="flex h-full flex-col items-center">
		<div
			class="shrink-0 mb-1 text-center text-xs leading-tight break-all {colorCodingEnabled ? 'text-blue-400' : 'text-text-muted'}"
			title={selectedFile}
		>
			Selected: {selectedFile}
		</div>
		<div class="flex min-h-0 flex-1 items-center justify-center overflow-hidden">
			<img
				src={fileUrl}
				alt="Selected"
				class="max-h-full max-w-full rounded-md object-contain object-right {colorCodingEnabled ? 'border-t-2 border-blue-500' : ''}"
				onerror={onMediaError}
			/>
		</div>
		<div
			class="shrink-0 mt-1 text-center text-xs leading-tight break-all text-text-muted"
		>
			{formatBytes(selectedFileSize)}{selectedFileDims ? ` • ${selectedFileDims}` : ''}
		</div>
	</div>
	<div class="flex h-full flex-col items-center">
		<div
			class="shrink-0 mb-1 text-center text-xs leading-tight break-all {colorCodingEnabled ? 'text-amber-400' : 'text-text-muted'}"
			title={compareTarget}
		>
			Comparing: {compareTarget}
		</div>
		<div class="flex min-h-0 flex-1 items-center justify-center overflow-hidden">
			<img
				src={compareUrl}
				alt="Comparing"
				class="max-h-full max-w-full rounded-md object-contain object-left {colorCodingEnabled ? 'border-t-2 border-amber-500' : ''}"
			/>
		</div>
		<div
			class="shrink-0 mt-1 text-center text-xs leading-tight break-all text-text-muted"
		>
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>