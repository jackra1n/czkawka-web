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

	let leftImgWidth = $state(0);
	let rightImgWidth = $state(0);
</script>

<div class="flex min-h-0 flex-1 items-center justify-center gap-2 bg-bg p-2">
	<div class="flex max-h-full flex-col items-center">
		<div
			class="mb-1 text-center text-xs leading-tight break-all {colorCodingEnabled ? 'text-blue-400' : 'text-text-muted'}"
			style:max-width="{leftImgWidth || undefined}px"
			title={selectedFile}
		>
			Selected: {selectedFile}
		</div>
		<img
			bind:clientWidth={leftImgWidth}
			src={fileUrl}
			alt="Selected"
			class="max-h-full max-w-full rounded-md object-contain {colorCodingEnabled ? 'border-t-2 border-blue-500' : ''}"
			onerror={onMediaError}
		/>
		<div
			class="mt-1 text-center text-xs leading-tight break-all text-text-muted"
			style:max-width="{leftImgWidth || undefined}px"
		>
			{formatBytes(selectedFileSize)}{selectedFileDims ? ` • ${selectedFileDims}` : ''}
		</div>
	</div>
	<div class="flex max-h-full flex-col items-center">
		<div
			class="mb-1 text-center text-xs leading-tight break-all {colorCodingEnabled ? 'text-amber-400' : 'text-text-muted'}"
			style:max-width="{rightImgWidth || undefined}px"
			title={compareTarget}
		>
			Comparing: {compareTarget}
		</div>
		<img
			bind:clientWidth={rightImgWidth}
			src={compareUrl}
			alt="Comparing"
			class="max-h-full max-w-full rounded-md object-contain {colorCodingEnabled ? 'border-t-2 border-amber-500' : ''}"
		/>
		<div
			class="mt-1 text-center text-xs leading-tight break-all text-text-muted"
			style:max-width="{rightImgWidth || undefined}px"
		>
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>
