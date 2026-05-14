<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ScannedFile, ScanProgress } from '$lib/api';
	import { formatBytes, formatDuration, formatDate } from '$lib/utils';

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
	});

	type ListItem = { type: 'file'; file: ScannedFile; size: number } | { type: 'separator' };

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

	let listItems = $derived(buildListItems(scanResults));

	$effect(() => {
		if (scanState === 'running') {
			selectedIndex = -1;
			checkedFiles.clear();
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
		if (item.type === 'file') {
			onSelectFile(item.file.path, item.size);
		} else {
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

	function scrollToIndex(index: number) {
		const el = document.getElementById(`scan-item-${index}`);
		if (el) {
			el.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
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
						<div class="relative flex items-center">
							{col.label}
							{#if ci < colDefs.length - 1}
								<div
									class="absolute top-0 right-0 bottom-0 z-10 w-2 cursor-col-resize hover:bg-accent/20"
									role="button"
									tabindex="-1"
									aria-label="Resize column"
									onmousedown={(e) => startResize(e, ci)}
								></div>
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
			<div class="flex min-w-full flex-col">
				{#each listItems as item, i (item.type === 'file' ? item.file.path : `sep-${i}`)}
					{#if item.type === 'separator'}
						<div
							id="scan-item-{i}"
							class="h-4 cursor-default {selectedIndex === i ? 'bg-surface-raised/30' : ''}"
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
							class="grid cursor-pointer gap-3 px-4 py-2 text-sm transition-colors {selectedIndex ===
							i
								? 'bg-surface-raised'
								: 'hover:bg-surface-raised/50'}"
							style="grid-template-columns: {gridCols()};"
							role="option"
							tabindex="-1"
							aria-selected={selectedIndex === i}
							onclick={() => handleSelect(i)}
							onkeydown={() => {}}
						>
							{#each colDefs as col (col.key)}
								{#if col.key === 'checkbox'}
									<div class="flex items-center justify-center">
										<input
											type="checkbox"
											checked={checkedFiles.has(item.file.path)}
											onclick={(e) => {
												e.stopPropagation();
												toggleCheck(item.file.path);
												handleSelect(i);
											}}
										/>
									</div>
								{:else if col.key === 'size'}
									<div class="flex items-center font-medium text-text">
										{formatBytes(item.size)}
									</div>
								{:else if col.key === 'dimensions'}
									<div class="flex items-center font-medium text-text">
										{item.file.dimensions ?? ''}
									</div>
								{:else if col.key === 'similarity'}
									<div class="flex items-center font-medium text-text">
										{item.file.similarity ?? ''}
									</div>
								{:else if col.key === 'filename'}
									<div class="flex min-w-0 items-center" title={name}>
										<span class="truncate text-text">{name}</span>
									</div>
								{:else if col.key === 'path'}
									<div
										class="flex min-w-0 items-center font-mono text-xs text-text-muted"
										title={dir}
									>
										<span class="truncate">{dir}</span>
									</div>
								{:else if col.key === 'modified'}
									<div class="flex items-center text-xs text-text-muted">
										{formatDate(item.file.modified_date)}
									</div>
								{/if}
							{/each}
						</div>
					{/if}
				{/each}
			</div>
		{/if}
	{/if}
</div>
