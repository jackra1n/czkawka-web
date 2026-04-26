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

<div class="flex min-h-0 flex-1 items-center justify-center gap-2 bg-bg p-2">
	<div class="flex max-h-full flex-col items-center">
		<div
			class="mb-1 text-center text-xs leading-tight break-all text-text-muted"
			style:max-width="{leftImgWidth || undefined}px"
			title={selectedFile}
		>
			{selectedFile}
		</div>
		<img
			bind:clientWidth={leftImgWidth}
			src={fileUrl}
			alt="Selected"
			class="max-h-full max-w-full rounded-md object-contain"
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
			class="mb-1 text-center text-xs leading-tight break-all text-text-muted"
			style:max-width="{rightImgWidth || undefined}px"
			title={compareTarget}
		>
			{compareTarget}
		</div>
		<img
			bind:clientWidth={rightImgWidth}
			src={compareUrl}
			alt="Compare"
			class="max-h-full max-w-full rounded-md object-contain"
		/>
		<div
			class="mt-1 text-center text-xs leading-tight break-all text-text-muted"
			style:max-width="{rightImgWidth || undefined}px"
		>
			{formatBytes(selectedFileSize)}{compareFileDims ? ` • ${compareFileDims}` : ''}
		</div>
	</div>
</div>
