<script lang="ts">
	import { Search, ChevronDown, File } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults } from '$lib/api';
	import { formatBytes, formatDuration } from '$lib/utils';

	let {
		scanState,
		scanError,
		scanResults,
		onSelectFile
	}: {
		scanState: 'idle' | 'running' | 'completed' | 'error';
		scanError: string;
		scanResults: ScanResults | null;
		onSelectFile: (file: string, size: number) => void;
	} = $props();

	let expandedGroups = $state<SvelteSet<number>>(new SvelteSet());

	$effect(() => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		scanResults;
		expandedGroups.clear();
	});

	function toggleGroup(index: number) {
		if (expandedGroups.has(index)) {
			expandedGroups.delete(index);
		} else {
			expandedGroups.add(index);
		}
	}
</script>

<div class="flex flex-1 flex-col min-h-0 overflow-auto">
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
			<div class="rounded-lg border border-danger/30 bg-danger/10 px-4 py-3 text-sm text-danger max-w-md w-full text-center">
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
				<div class="grid grid-cols-[120px_1fr_80px_40px] gap-4 border-b border-border px-4 py-2 text-xs font-medium text-text-muted uppercase tracking-wider">
					<div>Size</div>
					<div>Hash</div>
					<div class="text-right">Files</div>
					<div></div>
				</div>
				{#each scanResults.groups as group, i (group.hash)}
					<div class="border-b border-border">
						<button
							class="grid w-full grid-cols-[120px_1fr_80px_40px] items-center gap-4 px-4 py-3 text-left transition-colors hover:bg-surface-raised"
							onclick={() => toggleGroup(i)}
						>
							<span class="text-sm font-medium text-text">{formatBytes(group.size)}</span>
							<span class="truncate font-mono text-xs text-text-muted">{group.hash}</span>
							<span class="text-right text-sm text-text-muted">{group.files.length}</span>
							<div class="flex justify-center">
							<ChevronDown
								class="h-4 w-4 text-text-muted transition-transform duration-200 {expandedGroups.has(i) ? 'rotate-180' : ''}"
							/>
							</div>
						</button>

						{#if expandedGroups.has(i)}
							<div class="bg-surface/40">
								{#each group.files as file (file)}
									<button
										class="flex w-full items-center gap-3 px-4 py-2 pl-8 text-left text-sm text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
										onclick={() => onSelectFile(file, group.size)}
									>
										<File class="h-3.5 w-3.5 shrink-0 opacity-60" />
										<span class="truncate font-mono text-xs">{file}</span>
									</button>
								{/each}
							</div>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
	{/if}
</div>
