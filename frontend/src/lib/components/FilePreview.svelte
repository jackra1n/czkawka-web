<script lang="ts">
	import { X, Maximize2, Minimize2, GripVertical } from 'lucide-svelte';
	import { formatBytes, getPreviewType } from '$lib/utils';
	import { getFileUrl, fetchFileText } from '$lib/api';
	import type { ScannedFile } from '$lib/api';
	import ErrorIcon from './ErrorIcon.svelte';
	import MediaPreview from './MediaPreview.svelte';
	import CompareToolbar from './compare/CompareToolbar.svelte';
	import SideBySideCompare from './compare/SideBySideCompare.svelte';
	import SwipeCompare from './compare/SwipeCompare.svelte';
	import OnionSkinCompare from './compare/OnionSkinCompare.svelte';

	type CompareMode = 'single' | 'side-by-side' | 'swipe' | 'onion';

	let {
		selectedFile,
		selectedFileSize,
		groupFiles,
		onClose
	}: {
		selectedFile: string;
		selectedFileSize: number;
		groupFiles: ScannedFile[];
		onClose: () => void;
	} = $props();

	let textContent = $state('');
	let textLoading = $state(false);
	let textError = $state('');
	let mediaError = $state(false);
	let isMaximized = $state(false);

	let previewType = $derived(getPreviewType(selectedFile));
	let fileUrl = $derived(getFileUrl(selectedFile));

	// Comparison state
	const COMPARE_MODE_KEY = 'filePreviewCompareMode';
	let compareMode = $state<CompareMode>('single');
	let compareTarget = $state<string | null>(null);
	let swipePercent = $state(50);
	let onionOpacity = $state(0.5);

	let imageSiblings = $derived(groupFiles.filter((f) => getPreviewType(f.path) === 'image'));

	let compareUrl = $derived(compareTarget ? getFileUrl(compareTarget) : '');

	let selectedFileDims = $derived(
		groupFiles.find((f) => f.path === selectedFile)?.dimensions ?? ''
	);
	let compareFileDims = $derived(
		compareTarget ? (groupFiles.find((f) => f.path === compareTarget)?.dimensions ?? '') : ''
	);

	$effect(() => {
		if (typeof window === 'undefined') return;
		const saved = localStorage.getItem(COMPARE_MODE_KEY);
		if (saved && ['single', 'side-by-side', 'swipe', 'onion'].includes(saved)) {
			compareMode = saved as CompareMode;
		}
	});

	$effect(() => {
		const siblings = imageSiblings;
		const current = selectedFile;
		if (siblings.length > 1) {
			const firstOther = siblings.find((f) => f.path !== current);
			compareTarget = firstOther?.path ?? siblings[0]?.path ?? null;
		} else {
			compareTarget = null;
		}
		swipePercent = 50;
		onionOpacity = 0.5;
	});

	function setCompareMode(mode: CompareMode) {
		compareMode = mode;
		if (typeof window !== 'undefined') {
			localStorage.setItem(COMPARE_MODE_KEY, mode);
		}
	}

	function setCompareTarget(path: string) {
		compareTarget = path;
	}

	function toggleMaximize() {
		isMaximized = !isMaximized;
	}

	// Panel resize
	const MIN_WIDTH = 200;
	const MAX_WIDTH_FRACTION = 0.6;
	const STORAGE_KEY = 'filePreviewWidth';
	let panelWidth = $state(288);

	$effect(() => {
		if (typeof window === 'undefined') return;
		const saved = localStorage.getItem(STORAGE_KEY);
		if (saved) {
			const parsed = parseInt(saved, 10);
			if (!isNaN(parsed)) {
				panelWidth = Math.max(MIN_WIDTH, Math.min(window.innerWidth * MAX_WIDTH_FRACTION, parsed));
			}
		}
	});

	function startResize(e: MouseEvent) {
		e.preventDefault();
		const startX = e.clientX;
		const startWidth = panelWidth;
		const body = document.body;
		const originalUserSelect = body.style.userSelect;
		body.style.userSelect = 'none';

		function onMouseMove(e: MouseEvent) {
			const delta = startX - e.clientX;
			const maxWidth = window.innerWidth * MAX_WIDTH_FRACTION;
			panelWidth = Math.max(MIN_WIDTH, Math.min(maxWidth, startWidth + delta));
		}

		function onMouseUp() {
			window.removeEventListener('mousemove', onMouseMove);
			window.removeEventListener('mouseup', onMouseUp);
			body.style.userSelect = originalUserSelect;
			localStorage.setItem(STORAGE_KEY, String(panelWidth));
		}

		window.addEventListener('mousemove', onMouseMove);
		window.addEventListener('mouseup', onMouseUp);
	}

	// Swipe drag
	function startSwipeDrag(e: MouseEvent) {
		e.preventDefault();
		const container = (e.currentTarget as HTMLElement).parentElement;
		if (!container) return;
		const rect = container.getBoundingClientRect();

		function onMove(e: MouseEvent) {
			const x = e.clientX - rect.left;
			swipePercent = Math.max(0, Math.min(100, (x / rect.width) * 100));
		}

		function onUp() {
			window.removeEventListener('mousemove', onMove);
			window.removeEventListener('mouseup', onUp);
		}

		window.addEventListener('mousemove', onMove);
		window.addEventListener('mouseup', onUp);
	}

	// Text loading
	$effect(() => {
		const file = selectedFile;
		const type = previewType;
		textContent = '';
		textError = '';
		mediaError = false;

		if (type !== 'text') return;

		textLoading = true;
		const controller = new AbortController();

		fetchFileText(file, controller.signal)
			.then((content) => {
				textContent = content;
			})
			.catch((err) => {
				if (err.name !== 'AbortError') {
					textError = err.message ?? 'Failed to load file';
				}
			})
			.finally(() => {
				textLoading = false;
			});

		return () => {
			controller.abort();
		};
	});

	$effect(() => {
		if (!isMaximized) return;
		function onKey(e: KeyboardEvent) {
			if (e.key === 'Escape') isMaximized = false;
		}
		window.addEventListener('keydown', onKey);
		return () => window.removeEventListener('keydown', onKey);
	});

	function handleMediaError() {
		mediaError = true;
	}

	function setDefaultVolume(e: Event) {
		const el = e.currentTarget as HTMLVideoElement | HTMLAudioElement;
		el.volume = 0.3;
	}
</script>

<aside
	class="{isMaximized ? 'fixed inset-0 z-50' : 'relative shrink-0'} flex flex-col border-l border-border bg-surface"
	style:width={isMaximized ? undefined : `${panelWidth}px`}
>
	{#if !isMaximized}
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<div
			class="group absolute top-0 bottom-0 -left-2 z-20 flex w-4 cursor-col-resize items-center justify-center"
			onmousedown={startResize}
			role="separator"
			aria-label="Resize preview panel"
			aria-orientation="vertical"
		>
			<div class="flex h-8 w-5 items-center justify-center rounded-full bg-black/50 transition-colors group-hover:bg-black/70">
				<GripVertical class="h-4 w-4 text-text-muted opacity-75 transition-opacity group-hover:opacity-100" />
			</div>
		</div>
	{/if}

	<div class="flex items-center justify-between border-b border-border px-4 py-3">
		<span class="text-sm font-medium">Preview</span>
		<div class="flex items-center gap-1">
			<button
				onclick={toggleMaximize}
				title={isMaximized ? 'Minimize' : 'Maximize'}
				class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
			>
				{#if isMaximized}
					<Minimize2 class="h-4 w-4" />
				{:else}
					<Maximize2 class="h-4 w-4" />
				{/if}
			</button>
			<button
				onclick={onClose}
				class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
			>
				<X class="h-4 w-4" />
			</button>
		</div>
	</div>

	<div class="flex min-h-0 flex-1 flex-col">
		{#if previewType === 'image'}
			{#if mediaError}
				<ErrorIcon label="Failed to load image" />
			{:else}
				{#if imageSiblings.length > 1}
					<CompareToolbar
						{selectedFile}
						{imageSiblings}
						{compareMode}
						{compareTarget}
						{setCompareMode}
						{setCompareTarget}
					/>
				{/if}

				{#if compareMode === 'single' || imageSiblings.length < 2}
					<div class="flex flex-1 items-center justify-center bg-bg p-4">
						<img
							src={fileUrl}
							alt="Preview"
							class="max-h-full max-w-full rounded-md object-contain"
							onerror={handleMediaError}
						/>
					</div>
				{:else if compareMode === 'side-by-side'}
					<SideBySideCompare
						{fileUrl}
						{selectedFile}
						{selectedFileSize}
						{selectedFileDims}
						{compareUrl}
						compareTarget={compareTarget ?? ''}
						{compareFileDims}
						onMediaError={handleMediaError}
					/>
				{:else if compareMode === 'swipe'}
					<SwipeCompare
						{fileUrl}
						{selectedFile}
						{selectedFileSize}
						{selectedFileDims}
						{compareUrl}
						compareTarget={compareTarget ?? ''}
						{compareFileDims}
						{swipePercent}
						onSwipeDrag={startSwipeDrag}
					/>
				{:else if compareMode === 'onion'}
					<OnionSkinCompare
						{fileUrl}
						{selectedFile}
						{selectedFileSize}
						{selectedFileDims}
						{compareUrl}
						compareTarget={compareTarget ?? ''}
						{compareFileDims}
						{onionOpacity}
					/>
				{/if}
			{/if}
		{:else}
			<MediaPreview
				{previewType}
				{fileUrl}
				{mediaError}
				{textContent}
				{textLoading}
				{textError}
				onMediaError={handleMediaError}
				onDefaultVolume={setDefaultVolume}
			/>
		{/if}

		<div class="shrink-0 space-y-3 border-t border-border p-4">
			<div>
				<p class="mb-1 text-xs font-medium text-text-muted">Path</p>
				<p class="font-mono text-xs leading-relaxed break-all text-text">{selectedFile}</p>
			</div>
			<div>
				<p class="mb-1 text-xs font-medium text-text-muted">Size</p>
				<p class="text-sm text-text">{formatBytes(selectedFileSize)}</p>
			</div>
		</div>
	</div>
</aside>
