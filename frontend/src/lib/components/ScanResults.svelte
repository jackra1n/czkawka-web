<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, DuplicateFile } from '$lib/api';
	import { formatBytes, formatDuration, formatDate } from '$lib/utils';

	let {
		scanState,
		scanError,
		scanResults,
		onSelectFile
	}: {
		scanState: 'idle' | 'running' | 'completed' | 'error';
		scanError: string;
		scanResults: ScanResults | null;
		onSelectFile: (file: string | null, size: number) => void;
	} = $props();

	let containerEl = $state<HTMLDivElement | null>(null);
	let selectedIndex = $state(-1);
	let checkedFiles = $state<SvelteSet<string>>(new SvelteSet());

	type ListItem =
		| { type: 'file'; file: DuplicateFile; size: number }
		| { type: 'separator' };

	function buildListItems(results: ScanResults | null): ListItem[] {
		if (!results) return [];
		const items: ListItem[] = [];
		for (let gi = 0; gi < results.groups.length; gi++) {
			const group = results.groups[gi];
			for (const file of group.files) {
				items.push({ type: 'file', file, size: group.size });
			}
			if (gi < results.groups.length - 1) {
				items.push({ type: 'separator' });
			}
		}
		return items;
	}

	let listItems = $derived(buildListItems(scanResults));

	$effect(() => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		scanResults;
		selectedIndex = -1;
		checkedFiles.clear();
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
</script>

<div
	bind:this={containerEl}
	class="flex flex-1 flex-col min-h-0 overflow-auto outline-none"
	tabindex="0"
	onkeydown={handleKeyDown}
>
	{#if scanState === 'idle' && !scanResults}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 text-text-muted">
			<Search class="h-10 w-10 opacity-30" />
			<p class="text-sm">Enter directories and click Search to begin</p>
		</div>
	{:else if scanState === 'running'}
		<div class="flex flex-1 flex-col items-center justify-center gap-4">
			<div class="h-8 w-8 animate-spin rounded-full border-2 border-border border-t-accent"></div>
			<p class="text-sm text-text-muted">Scanning for duplicates…</p>
		</div>
	{:else if scanState === 'error'}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 p-8">
			<div
				class="rounded-lg border border-danger/30 bg-danger/10 px-4 py-3 text-sm text-danger max-w-md w-full text-center"
			>
				{scanError}
			</div>
		</div>
	{:else if scanResults}
		<!-- Stats bar -->
		<div class="flex items-center gap-6 border-b border-border px-4 py-2 text-xs text-text-muted">
			<span>Groups: <strong class="text-text">{scanResults.total_duplicate_groups}</strong></span>
			<span>Files: <strong class="text-text">{scanResults.total_duplicate_files}</strong></span>
			<span>Wasted: <strong class="text-text">{formatBytes(scanResults.wasted_space_bytes)}</strong></span>
			<span>Duration: <strong class="text-text">{formatDuration(scanResults.scanning_time_ms)}</strong></span>
		</div>

		{#if scanResults.groups.length === 0}
			<div class="flex flex-1 flex-col items-center justify-center text-sm text-text-muted">
				No duplicates found.
			</div>
		{:else}
			<!-- Table -->
			<div class="flex flex-col">
				<div
					class="grid grid-cols-[32px_100px_1fr_1.5fr_140px] gap-3 border-b border-border px-4 py-2 text-xs font-medium text-text-muted uppercase tracking-wider"
				>
					<div></div>
					<div>Size</div>
					<div>Filename</div>
					<div>Path</div>
					<div>Modified</div>
				</div>
				{#each listItems as item, i (item.type === 'file' ? item.file.path : `sep-${i}`)}
					{#if item.type === 'separator'}
						<div
							id="scan-item-{i}"
							class="h-4 cursor-default {selectedIndex === i ? 'bg-surface-raised/30' : ''}"
							onclick={() => handleSelect(i)}
						></div>
					{:else}
						{@const { name, dir } = splitPath(item.file.path)}
						<div
							id="scan-item-{i}"
							class="grid grid-cols-[32px_100px_1fr_1.5fr_140px] gap-3 px-4 py-2 text-sm cursor-pointer transition-colors {selectedIndex === i ? 'bg-surface-raised' : 'hover:bg-surface-raised/50'}"
							onclick={() => handleSelect(i)}
						>
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
							<div class="flex items-center text-text font-medium">{formatBytes(item.size)}</div>
							<div class="flex items-center truncate text-text" title={name}>{name}</div>
							<div class="flex items-center truncate font-mono text-xs text-text-muted" title={dir}>
								{dir}
							</div>
							<div class="flex items-center text-xs text-text-muted">
								{formatDate(item.file.modified_date)}
							</div>
						</div>
					{/if}
				{/each}
			</div>
		{/if}
	{/if}
</div>
