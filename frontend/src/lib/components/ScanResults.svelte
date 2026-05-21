<script lang="ts">
	import { Search, ArrowUp, ArrowDown } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ScannedFile, ScanProgress } from '$lib/api';
	import { formatBytes, formatDuration, formatDate } from '$lib/utils';
	import { computeVirtualLayout, findVisibleRange } from '$lib/virtualList';
	import Tooltip from './ui/Tooltip.svelte';
	import Checkbox from './ui/Checkbox.svelte';

	let {
		scanState,
		scanError,
		scanResults,
		scanProgress,
		onSelectFile,
		checkedFiles,
		activeTool
	}: {
		scanState: 'idle' | 'running' | 'completed' | 'error';
		scanError: string;
		scanResults: ScanResults | null;
		scanProgress: ScanProgress | null;
		onSelectFile: (file: string | null, size: number) => void;
		checkedFiles: SvelteSet<string>;
		activeTool: string;
	} = $props();

	let containerEl = $state<HTMLDivElement | null>(null);
	let selectedIndex = $state(-1);
	let sortKey = $state<string | null>(null);
	let sortDesc = $state(false);
	let selectedPath = $state<string | null>(null);

	type ColDef = { key: string; label: string; width: number; minWidth: number };

	const TOOL_COLS: Record<string, ColDef[]> = {
		duplicates: [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 240, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 360, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'empty-files': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'filename', label: 'Filename', width: 300, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 400, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'empty-folders': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'filename', label: 'Folder', width: 300, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 400, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'big-files': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 240, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 360, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		temporary: [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 240, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 360, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'similar-images': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'dimensions', label: 'Dimensions', width: 100, minWidth: 50 },
			{ key: 'similarity', label: 'Similarity', width: 100, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'similar-videos': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'dimensions', label: 'Dimensions', width: 100, minWidth: 50 },
			{ key: 'similarity', label: 'Duration', width: 100, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'same-music': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'Track', width: 180, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'invalid-symlinks': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'Destination', width: 240, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'broken-files': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'Error', width: 240, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'bad-extensions': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'Extension', width: 200, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'exif-remover': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'EXIF Tags', width: 120, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 220, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		],
		'bad-names': [
			{ key: 'checkbox', label: '', width: 40, minWidth: 20 },
			{ key: 'size', label: 'Size', width: 110, minWidth: 50 },
			{ key: 'similarity', label: 'New Name', width: 220, minWidth: 50 },
			{ key: 'filename', label: 'Filename', width: 200, minWidth: 50 },
			{ key: 'path', label: 'Path', width: 300, minWidth: 50 },
			{ key: 'modified', label: 'Modified', width: 150, minWidth: 50 }
		]
	};

	let colDefs = $derived(TOOL_COLS[activeTool] ?? TOOL_COLS.duplicates);
	let colWidths = $state<number[]>([]);

	$effect(() => {
		colWidths = colDefs.map((c) => c.width);
		selectedIndex = -1;
		selectedPath = null;
	});

	$effect(() => {
		if (activeTool || scanState === 'running') {
			sortKey = null;
			sortDesc = false;
		}
	});

	type ListItem = { type: 'file'; file: ScannedFile; size: number } | { type: 'separator' };

	function getPixelCount(dimensions?: string): number {
		if (!dimensions) return 0;
		const match = dimensions.match(/^(\d+)x(\d+)$/);
		if (!match) return 0;
		return parseInt(match[1], 10) * parseInt(match[2], 10);
	}

	function compareValues(a: unknown, b: unknown): number {
		if (a === b) return 0;
		if (a == null) return 1;
		if (b == null) return -1;

		const aNum = typeof a === 'number' ? a : (typeof a === 'string' && a.trim() !== '' ? Number(a.replace(/%$/, '')) : NaN);
		const bNum = typeof b === 'number' ? b : (typeof b === 'string' && b.trim() !== '' ? Number(b.replace(/%$/, '')) : NaN);

		if (!isNaN(aNum) && !isNaN(bNum)) {
			return aNum - bNum;
		}

		return String(a).localeCompare(String(b), undefined, { numeric: true, sensitivity: 'base' });
	}

	function getSortValue(file: ScannedFile, group: { size: number }, key: string): string | number {
		switch (key) {
			case 'size':
				return file.size ?? group.size ?? 0;
			case 'filename':
				return splitPath(file.path).name.toLowerCase();
			case 'path':
				return splitPath(file.path).dir.toLowerCase();
			case 'modified':
				return file.modified_date ?? 0;
			case 'dimensions':
				return getPixelCount(file.dimensions);
			case 'similarity':
				return file.similarity ?? '';
			default:
				return '';
		}
	}

	function sortScanResults(results: ScanResults | null, key: string | null, desc: boolean): ScanResults | null {
		if (!results || !key) return results;

		const sortedGroups = results.groups.map((group) => {
			const files = [...group.files];
			files.sort((a, b) => {
				const valA = getSortValue(a, group, key);
				const valB = getSortValue(b, group, key);
				const cmp = compareValues(valA, valB);
				return desc ? -cmp : cmp;
			});
			return {
				...group,
				files
			};
		});

		sortedGroups.sort((gA, gB) => {
			const fileA = gA.files[0];
			const fileB = gB.files[0];

			if (!fileA && !fileB) return 0;
			if (!fileA) return desc ? -1 : 1;
			if (!fileB) return desc ? 1 : -1;

			const valA = getSortValue(fileA, gA, key);
			const valB = getSortValue(fileB, gB, key);
			const cmp = compareValues(valA, valB);
			return desc ? -cmp : cmp;
		});

		return {
			...results,
			groups: sortedGroups
		};
	}

	function toggleSort(key: string) {
		if (sortKey === key) {
			if (!sortDesc) {
				sortDesc = true;
			} else {
				sortKey = null;
				sortDesc = false;
			}
		} else {
			sortKey = key;
			sortDesc = false;
		}
	}

	function buildListItems(results: ScanResults | null): ListItem[] {
		if (!results) return [];
		const items: ListItem[] = [];
		const showSeparators =
			activeTool !== 'empty-folders' &&
			activeTool !== 'big-files' &&
			activeTool !== 'empty-files' &&
			activeTool !== 'temporary' &&
			activeTool !== 'invalid-symlinks' &&
			activeTool !== 'broken-files' &&
			activeTool !== 'bad-extensions' &&
			activeTool !== 'exif-remover' &&
			activeTool !== 'bad-names';
		for (let gi = 0; gi < results.groups.length; gi++) {
			const group = results.groups[gi];
			for (const file of group.files) {
				items.push({ type: 'file', file, size: group.size });
			}
			if (showSeparators && gi < results.groups.length - 1) {
				items.push({ type: 'separator' });
			}
		}
		return items;
	}

	let sortedResults = $derived(sortScanResults(scanResults, sortKey, sortDesc));
	let listItems = $derived(buildListItems(sortedResults));

	let layout = $derived(computeVirtualLayout(listItems, (item) => (item.type === 'separator' ? 16 : 36)));

	let scrollTop = $state(0);
	let clientHeight = $state(0);

	function handleScroll() {
		if (containerEl) {
			scrollTop = containerEl.scrollTop;
		}
	}

	$effect(() => {
		if (containerEl) {
			scrollTop = containerEl.scrollTop;
			clientHeight = containerEl.clientHeight;

			const resizeObserver = new ResizeObserver(() => {
				if (containerEl) {
					clientHeight = containerEl.clientHeight;
				}
			});
			resizeObserver.observe(containerEl);

			return () => {
				resizeObserver.disconnect();
			};
		}
	});

	let visibleRange = $derived.by(() => {
		const viewTop = Math.max(0, scrollTop - 1000);
		const viewBottom = scrollTop + clientHeight + 1000;
		return findVisibleRange(layout.items, viewTop, viewBottom);
	});

	let visibleItems = $derived(layout.items.slice(visibleRange.start, visibleRange.end + 1));

	$effect(() => {
		if (scanState === 'running') {
			selectedIndex = -1;
			selectedPath = null;
			checkedFiles.clear();
		}
	});

	$effect(() => {
		if (selectedPath) {
			const idx = listItems.findIndex((item) => item.type === 'file' && item.file.path === selectedPath);
			if (idx !== -1) {
				selectedIndex = idx;
			} else {
				selectedIndex = -1;
				selectedPath = null;
			}
		}
	});

	function toggleCheck(path: string) {
		if (checkedFiles.has(path)) {
			checkedFiles.delete(path);
		} else {
			checkedFiles.add(path);
		}
	}

	function handleSelect(index: number) {
		selectedIndex = index;
		containerEl?.focus();
		const item = listItems[index];
		if (item && item.type === 'file') {
			selectedPath = item.file.path;
			onSelectFile(item.file.path, item.size);
		} else {
			selectedPath = null;
			onSelectFile(null, 0);
		}
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			const next = selectedIndex + 1;
			if (next < listItems.length) {
				handleSelect(next);
				scrollToIndex(next);
			}
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			const prev = selectedIndex - 1;
			if (prev >= 0) {
				handleSelect(prev);
				scrollToIndex(prev);
			}
		} else if (e.key === ' ') {
			e.preventDefault();
			const item = listItems[selectedIndex];
			if (item?.type === 'file') {
				toggleCheck(item.file.path);
			}
		}
	}

	function getHeaderHeight(): number {
		const header = containerEl?.querySelector('.sticky');
		return header ? header.getBoundingClientRect().height : 0;
	}

	function scrollToIndex(index: number) {
		if (!containerEl || index < 0 || index >= layout.items.length) return;
		const item = layout.items[index];
		const headerHeight = getHeaderHeight();
		const currentScroll = containerEl.scrollTop;
		const viewportTop = currentScroll + headerHeight;
		const viewportBottom = currentScroll + clientHeight;

		const itemTop = item.top + headerHeight;
		const itemBottom = itemTop + item.height;

		if (itemTop < viewportTop) {
			containerEl.scrollTo({
				top: item.top,
				behavior: 'smooth'
			});
		} else if (itemBottom > viewportBottom) {
			containerEl.scrollTo({
				top: item.top + item.height + headerHeight - clientHeight,
				behavior: 'smooth'
			});
		}
	}

	function splitPath(fullPath: string): { name: string; dir: string } {
		const lastSlash = fullPath.lastIndexOf('/');
		const lastBackslash = fullPath.lastIndexOf('\\');
		const lastSep = Math.max(lastSlash, lastBackslash);
		if (lastSep === -1) return { name: fullPath, dir: '' };
		return { name: fullPath.slice(lastSep + 1), dir: fullPath.slice(0, lastSep) };
	}

	function startResize(e: MouseEvent, colIndex: number) {
		e.preventDefault();
		const startX = e.clientX;
		const startWidth = colWidths[colIndex];
		const minW = colDefs[colIndex].minWidth;
		const body = document.body;
		const originalUserSelect = body.style.userSelect;
		body.style.userSelect = 'none';

		function onMouseMove(ev: MouseEvent) {
			const delta = ev.clientX - startX;
			colWidths[colIndex] = Math.max(minW, startWidth + delta);
		}

		function onMouseUp() {
			window.removeEventListener('mousemove', onMouseMove);
			window.removeEventListener('mouseup', onMouseUp);
			body.style.userSelect = originalUserSelect;
		}

		window.addEventListener('mousemove', onMouseMove);
		window.addEventListener('mouseup', onMouseUp);
	}

	function gridCols(): string {
		return colWidths.map((w) => w + 'px').join(' ');
	}

	const SCAN_TEXTS: Record<string, { scanning: string; empty: string }> = {
		duplicates: { scanning: 'Scanning for duplicates…', empty: 'No duplicates found.' },
		'empty-files': { scanning: 'Scanning for empty files…', empty: 'No empty files found.' },
		'empty-folders': { scanning: 'Scanning for empty folders…', empty: 'No empty folders found.' },
		'big-files': { scanning: 'Scanning for big files…', empty: 'No big files found.' },
		'similar-images': {
			scanning: 'Scanning for similar images…',
			empty: 'No similar images found.'
		},
		'similar-videos': {
			scanning: 'Scanning for similar videos…',
			empty: 'No similar videos found.'
		},
		'same-music': { scanning: 'Scanning for same music…', empty: 'No same music found.' },
		'invalid-symlinks': {
			scanning: 'Scanning for invalid symlinks…',
			empty: 'No invalid symlinks found.'
		},
		'broken-files': { scanning: 'Scanning for broken files…', empty: 'No broken files found.' },
		'bad-extensions': {
			scanning: 'Scanning for bad extensions…',
			empty: 'No bad extensions found.'
		},
		'exif-remover': { scanning: 'Scanning for EXIF data…', empty: 'No files with EXIF found.' },
		'bad-names': { scanning: 'Scanning for bad names…', empty: 'No bad names found.' },
		temporary: { scanning: 'Scanning for temporary files…', empty: 'No temporary files found.' }
	};
	let scanningText = $derived(SCAN_TEXTS[activeTool]?.scanning ?? 'Scanning…');
	let emptyText = $derived(SCAN_TEXTS[activeTool]?.empty ?? 'Nothing found.');

	function progressPercent(p: ScanProgress): number {
		if (p.entries_to_check > 0) {
			return Math.min(99, Math.round((p.entries_checked / p.entries_to_check) * 100));
		}
		if (p.max_stage_idx > 0) {
			return Math.min(99, Math.round((p.current_stage_idx / (p.max_stage_idx + 1)) * 100));
		}
		return 0;
	}

	function progressDetail(p: ScanProgress): string {
		if (p.entries_to_check > 0) {
			return `${p.entries_checked.toLocaleString()} / ${p.entries_to_check.toLocaleString()}`;
		}
		if (p.entries_checked > 0) {
			return `${p.entries_checked.toLocaleString()} found`;
		}
		return '';
	}
</script>

<div
	bind:this={containerEl}
	class="flex min-h-0 flex-1 flex-col overflow-auto outline-none"
	tabindex="0"
	role="listbox"
	aria-label="Scan results"
	onkeydown={handleKeyDown}
	onscroll={handleScroll}
>
	{#if scanState === 'idle' && !scanResults}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 text-text-muted">
			<Search class="h-10 w-10 opacity-30" />
			<p class="text-sm">Enter directories and click Search to begin</p>
		</div>
	{:else if scanState === 'running'}
		<div class="flex flex-1 flex-col items-center justify-center gap-4 px-8">
			{#if scanProgress}
				<div class="w-full max-w-md space-y-3">
					<div class="flex items-center justify-between text-sm">
						<span class="text-text">{scanProgress.stage_label}</span>
						{#if scanProgress.max_stage_idx > 0}
							<span class="text-xs text-text-muted">
								Stage {scanProgress.current_stage_idx + 1} / {scanProgress.max_stage_idx + 1}
							</span>
						{/if}
					</div>
					<div class="h-2 w-full overflow-hidden rounded-full bg-surface-raised">
						<div
							class="h-full rounded-full bg-accent transition-all duration-300 ease-out"
							style="width: {progressPercent(scanProgress)}%"
						></div>
					</div>
					<div class="flex items-center justify-between text-xs text-text-muted">
						<span>{progressDetail(scanProgress)}</span>
						<span>{progressPercent(scanProgress)}%</span>
					</div>
				</div>
			{:else}
				<div class="h-8 w-8 animate-spin rounded-full border-2 border-border border-t-accent"></div>
				<p class="text-sm text-text-muted">{scanningText}</p>
			{/if}
		</div>
	{:else if scanState === 'error'}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 p-8">
			<div
				class="w-full max-w-md rounded-lg border border-danger/30 bg-danger/10 px-4 py-3 text-center text-sm text-danger"
			>
				{scanError}
			</div>
		</div>
	{:else if scanResults}
		<div class="sticky top-0 z-10 bg-surface">
			<!-- Stats bar -->
			<div class="flex items-center gap-6 border-b border-border px-4 py-2 text-xs text-text-muted">
				<span>Groups: <strong class="text-text">{scanResults.total_groups}</strong></span>
				<span>Items: <strong class="text-text">{scanResults.total_items}</strong></span>
				{#if activeTool !== 'empty-folders' && activeTool !== 'big-files' && activeTool !== 'empty-files' && activeTool !== 'temporary' && activeTool !== 'invalid-symlinks' && activeTool !== 'broken-files' && activeTool !== 'bad-extensions' && activeTool !== 'exif-remover' && activeTool !== 'bad-names'}
					<span
						>Wasted: <strong class="text-text">{formatBytes(scanResults.wasted_bytes)}</strong></span
					>
				{/if}
				<span
					>Duration: <strong class="text-text">{formatDuration(scanResults.scanning_time_ms)}</strong
					></span
				>
			</div>

			{#if scanResults.groups.length > 0}
				<!-- Table header -->
				<div
					class="grid gap-3 border-b border-border px-4 py-2 text-xs font-medium tracking-wider text-text-muted uppercase"
					style="grid-template-columns: {gridCols()};"
				>
					{#each colDefs as col, ci (col.key)}
						<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
						<div
							class="relative flex min-w-0 items-center select-none {col.key !== 'checkbox' ? 'cursor-pointer hover:text-text' : ''}"
							onclick={() => {
								if (col.key !== 'checkbox') {
									toggleSort(col.key);
								}
							}}
						>
							<span class="truncate">{col.label}</span>
							{#if sortKey === col.key}
								{#if sortDesc}
									<ArrowDown class="ml-1 h-3 w-3 shrink-0" />
								{:else}
									<ArrowUp class="ml-1 h-3 w-3 shrink-0" />
								{/if}
							{/if}
							{#if ci < colDefs.length - 1}
								<div
									class="group absolute top-0 right-0 bottom-0 z-10 flex w-3 cursor-col-resize items-center justify-center"
									role="button"
									tabindex="-1"
									aria-label="Resize column"
									onmousedown={(e) => {
										e.stopPropagation();
										startResize(e, ci);
									}}
									onclick={(e) => e.stopPropagation()}
								>
									<div class="h-6 w-px rounded-full bg-border transition-colors group-hover:bg-text-muted"></div>
								</div>
							{/if}
						</div>
					{/each}
				</div>
			{/if}
		</div>

		{#if scanResults.groups.length === 0}
			<div class="flex flex-1 flex-col items-center justify-center text-sm text-text-muted">
				{emptyText}
			</div>
		{:else}
			<!-- Table rows -->
			<div class="relative min-w-full shrink-0" style="height: {layout.totalHeight}px;">
				{#each visibleItems as renderItem (renderItem.item.type === 'file' ? renderItem.item.file.path : `sep-${renderItem.index}`)}
					{@const item = renderItem.item}
					{@const i = renderItem.index}
					<div style="position: absolute; top: {renderItem.top}px; left: 0; right: 0; height: {renderItem.height}px; overflow: hidden;">
						{#if item.type === 'separator'}
							<div
								id="scan-item-{i}"
								class="h-full cursor-default {selectedIndex === i ? 'bg-accent/10' : ''}"
								role="option"
								tabindex="-1"
								aria-selected={selectedIndex === i}
								onclick={() => handleSelect(i)}
								onkeydown={() => {}}
							></div>
						{:else}
							{@const { name, dir } = splitPath(item.file.path)}
							<div
								id="scan-item-{i}"
								class="grid h-full cursor-pointer gap-3 px-4 py-2 text-sm transition-colors {selectedIndex === i
									? 'bg-accent/15'
									: 'hover:bg-accent/10'}"
								style="grid-template-columns: {gridCols()};"
								role="option"
								tabindex="-1"
								aria-selected={selectedIndex === i}
								onclick={() => handleSelect(i)}
								onkeydown={() => {}}
							>
								{#each colDefs as col (col.key)}
									{#if col.key === 'checkbox'}
										<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
										<div class="flex items-center justify-center" onclick={(e) => e.stopPropagation()}>
											<Checkbox
												checked={checkedFiles.has(item.file.path)}
												onchange={() => {
													toggleCheck(item.file.path);
													handleSelect(i);
												}}
											/>
										</div>
									{:else if col.key === 'size'}
										<div class="truncate text-text">
											{formatBytes(item.size)}
										</div>
									{:else if col.key === 'dimensions'}
										<div class="truncate text-text">
											{item.file.dimensions ?? ''}
										</div>
									{:else if col.key === 'similarity'}
										<div class="truncate text-text">
											{item.file.similarity ?? ''}
										</div>
									{:else if col.key === 'filename'}
										<Tooltip class="flex min-w-0" content={name}>
											<div class="flex min-w-0 items-center">
												<span class="truncate text-text">{name}</span>
											</div>
										</Tooltip>
									{:else if col.key === 'path'}
										<Tooltip class="flex min-w-0" content={dir}>
											<div class="flex min-w-0 items-center font-mono text-xs text-text-muted">
												<span class="truncate">{dir}</span>
											</div>
										</Tooltip>
									{:else if col.key === 'modified'}
										<div class="truncate text-xs text-text-muted">
											{formatDate(item.file.modified_date)}
										</div>
									{/if}
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
