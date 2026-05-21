<script lang="ts">
	import { MoveHorizontal } from 'lucide-svelte';
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
		swipePercent,
		onSwipeDrag,
	}: {
		fileUrl: string;
		selectedFile: string;
		selectedFileSize: number;
		selectedFileDims: string;
		compareUrl: string;
		compareTarget: string;
		compareFileDims: string;
		swipePercent: number;
		onSwipeDrag: (e: MouseEvent) => void;
	} = $props();
</script>

<div class="flex min-h-0 flex-1 flex-col">
	<div class="mb-2 flex shrink-0 gap-2 px-2 pt-2">
		<Tooltip class="flex min-w-0 flex-1" content={selectedFile}>
			<div class="min-w-0 flex-1 truncate text-xs text-text-muted">
				Selected: {selectedFile}
			</div>
		</Tooltip>
		<Tooltip class="flex min-w-0 flex-1" content={compareTarget}>
			<div class="min-w-0 flex-1 truncate text-right text-xs text-text-muted">
				Comparing: {compareTarget}
			</div>
		</Tooltip>
	</div>
	<div class="relative flex min-h-0 flex-1 items-center justify-center bg-bg">
		<img src={compareUrl} alt="Comparing" class="max-h-full max-w-full object-contain" />
		<div
			class="pointer-events-none absolute inset-0 flex items-center justify-center"
			style="clip-path: inset(0 {100 - swipePercent}% 0 0)"
		>
			<img src={fileUrl} alt="Selected" class="max-h-full max-w-full object-contain" />
		</div>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="absolute top-0 bottom-0 z-10 w-0.5 cursor-ew-resize bg-white/80 shadow"
			style="left: {swipePercent}%"
			onmousedown={onSwipeDrag}
		>
			<div
				class="absolute top-1/2 left-1/2 flex h-5 w-5 -translate-x-1/2 -translate-y-1/2 items-center justify-center rounded-full bg-white shadow"
			>
				<MoveHorizontal class="h-3 w-3 text-text" />
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
