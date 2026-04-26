<script lang="ts">
	import { MoveHorizontal } from 'lucide-svelte';
	import { formatBytes } from '$lib/utils';

	let {
		fileUrl,
		selectedFile,
		selectedFileSize,
		selectedFileDims,
		compareUrl,
		compareTarget,
		compareFileDims,
		swipePercent,
		onSwipeDrag
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

<div class="flex flex-1 min-h-0 flex-col">
	<div class="flex shrink-0 gap-2 px-2 pt-2 mb-2">
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted" title={selectedFile}>Selected: {selectedFile}</div>
		<div class="flex-1 min-w-0 truncate text-xs text-text-muted text-right" title={compareTarget}>Compare: {compareTarget}</div>
	</div>
	<div class="relative flex-1 min-h-0 bg-bg flex items-center justify-center">
		<img src={compareUrl} alt="Compare" class="max-h-full max-w-full object-contain" />
		<div
			class="absolute inset-0 flex items-center justify-center pointer-events-none"
			style="clip-path: inset(0 {100 - swipePercent}% 0 0)"
		>
			<img src={fileUrl} alt="Selected" class="max-h-full max-w-full object-contain" />
		</div>
		<!-- svelte-ignore a11y_no_static_element_interactions -->
		<div
			class="absolute top-0 bottom-0 w-0.5 bg-white/80 shadow cursor-ew-resize z-10"
			style="left: {swipePercent}%"
			onmousedown={onSwipeDrag}
		>
			<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-5 h-5 rounded-full bg-white shadow flex items-center justify-center">
				<MoveHorizontal class="h-3 w-3 text-text" />
			</div>
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