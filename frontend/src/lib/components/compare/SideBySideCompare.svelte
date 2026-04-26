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
		onMediaError
	}: {
		fileUrl: string;
		selectedFile: string;
		selectedFileSize: number;
		selectedFileDims: string;
		compareUrl: string;
		compareTarget: string;
		compareFileDims: string;
		onMediaError: () => void;
	} = $props();

	let leftImgWidth = $state(0);
	let rightImgWidth = $state(0);
</script>

<div class="flex flex-1 min-h-0 gap-2 p-2 bg-bg items-center justify-center">
	<div class="flex flex-col items-center max-h-full">
		<div class="text-xs text-text-muted mb-1 break-all text-center leading-tight" style:max-width="{leftImgWidth || undefined}px" title={selectedFile}>{selectedFile}</div>
		<img bind:clientWidth={leftImgWidth} src={fileUrl} alt="Selected" class="max-h-full max-w-full object-contain rounded-md" onerror={onMediaError} />
		<div class="text-xs text-text-muted mt-1 break-all text-center leading-tight" style:max-width="{leftImgWidth || undefined}px">
			{formatBytes(selectedFileSize)}{selectedFileDims ? ` • ${selectedFileDims}` : ''}
		</div>
	</div>
	<div class="flex flex-col items-center max-h-full">
		<div class="text-xs text-text-muted mb-1 break-all text-center leading-tight" style:max-width="{rightImgWidth || undefined}px" title={compareTarget}>{compareTarget}</div>
		<img bind:clientWidth={rightImgWidth} src={compareUrl} alt="Compare" class="max-h-full max-w-full object-contain rounded-md" />
		<div class="text-xs text-text-muted mt-1 break-all text-center leading-tight" style:max-width="{rightImgWidth || undefined}px">
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>