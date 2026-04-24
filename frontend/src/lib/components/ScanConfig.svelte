<script lang="ts">
	import { Plus, X, Search } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ToolConfig } from '$lib/api';
	import ScanActions from './ScanActions.svelte';

	let {
		includedDirs = $bindable<string[]>(),
		excludedDirs = $bindable<string[]>(),
		excludedItems = $bindable<string>(),
		activeTool = $bindable<string>(),
		toolConfig = $bindable<ToolConfig>({}),
		scanState,
		scanResults,
		checkedFiles,
		onStartScan,
		onAddDir,
		onDelete
	}: {
		includedDirs: string[];
		excludedDirs: string[];
		excludedItems: string;
		activeTool: string;
		toolConfig: ToolConfig;
		scanState: 'idle' | 'running' | 'completed' | 'error';
		scanResults: ScanResults | null;
		checkedFiles: SvelteSet<string>;
		onStartScan: () => void;
		onAddDir: (target: 'include' | 'exclude') => void;
		onDelete: () => void;
	} = $props();

	let activeTab = $state<'directories' | 'items' | 'settings'>('directories');

	const HASH_ALGS = ['Mean', 'Gradient', 'Blockhash', 'VertGradient', 'DoubleGradient', 'Median'];
	const HASH_SIZES = [8, 16, 32, 64];
	const RESIZE_FILTERS = ['Lanczos3', 'Nearest', 'Triangle', 'Gaussian', 'CatmullRom'];
	const SEARCH_MODES = [
		{ value: 'biggest', label: 'Biggest' },
		{ value: 'smallest', label: 'Smallest' }
	];

	function addIncludedDir() {
		onAddDir('include');
	}

	function removeIncludedDir(index: number) {
		includedDirs = includedDirs.filter((_, i) => i !== index);
	}

	function addExcludedDir() {
		onAddDir('exclude');
	}

	function removeExcludedDir(index: number) {
		excludedDirs = excludedDirs.filter((_, i) => i !== index);
	}

	$effect(() => {
		if (activeTab === 'settings' && activeTool !== 'similar-images' && activeTool !== 'big-files') {
			activeTab = 'directories';
		}
	});
</script>

<div class="shrink-0 border-b border-border bg-surface p-4">
	<div class="mb-3 flex border-b border-border">
		<button
			type="button"
			onclick={() => (activeTab = 'directories')}
			class="relative px-3 py-2 text-xs font-medium transition-colors {activeTab === 'directories' ? 'text-accent' : 'text-text-muted hover:text-text'}"
		>
			Directories
			{#if activeTab === 'directories'}
				<span class="absolute bottom-0 left-0 right-0 h-0.5 bg-accent rounded-t-sm"></span>
			{/if}
		</button>
		<button
			type="button"
			onclick={() => (activeTab = 'items')}
			class="relative px-3 py-2 text-xs font-medium transition-colors {activeTab === 'items' ? 'text-accent' : 'text-text-muted hover:text-text'}"
		>
			Items
			{#if activeTab === 'items'}
				<span class="absolute bottom-0 left-0 right-0 h-0.5 bg-accent rounded-t-sm"></span>
			{/if}
		</button>
		{#if activeTool === 'similar-images' || activeTool === 'big-files'}
			<button
				type="button"
				onclick={() => (activeTab = 'settings')}
				class="relative px-3 py-2 text-xs font-medium transition-colors {activeTab === 'settings' ? 'text-accent' : 'text-text-muted hover:text-text'}"
			>
				Settings
				{#if activeTab === 'settings'}
					<span class="absolute bottom-0 left-0 right-0 h-0.5 bg-accent rounded-t-sm"></span>
				{/if}
			</button>
		{/if}
	</div>

	{#if activeTab === 'directories'}
		<div class="flex gap-4">
			<div class="flex flex-1 flex-col gap-2">
				<div class="flex items-center justify-between">
					<span class="text-xs font-medium text-text-muted">Included directories</span>
					<button
						type="button"
						onclick={addIncludedDir}
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
					>
						<Plus class="h-3.5 w-3.5" />
						Add
					</button>
				</div>
				<div class="flex max-h-40 flex-col gap-0.5 overflow-y-auto rounded-md border border-border bg-bg p-1 pr-0.5">
					{#if includedDirs.length === 0}
						<div class="flex items-center justify-center py-2 text-sm text-text-muted">
							No directories added
						</div>
					{:else}
						{#each includedDirs as dir, i (i)}
							<div class="flex items-center gap-1 rounded-sm px-2 py-1 hover:bg-surface">
								<span class="min-w-0 flex-1 truncate text-sm text-text" title={dir}>
									{dir}
								</span>
								<button
									type="button"
									onclick={() => removeIncludedDir(i)}
									class="flex shrink-0 items-center justify-center rounded p-0.5 text-text-muted transition-colors hover:text-danger"
									title="Remove"
								>
									<X class="h-3.5 w-3.5" />
								</button>
							</div>
						{/each}
					{/if}
				</div>
			</div>
			<div class="flex flex-1 flex-col gap-2">
				<div class="flex items-center justify-between">
					<span class="text-xs font-medium text-text-muted">Excluded directories</span>
					<button
						type="button"
						onclick={addExcludedDir}
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
					>
						<Plus class="h-3.5 w-3.5" />
						Add
					</button>
				</div>
				<div class="flex max-h-40 flex-col gap-0.5 overflow-y-auto rounded-md border border-border bg-bg p-1 pr-0.5">
					{#if excludedDirs.length === 0}
						<div class="flex items-center justify-center py-2 text-sm text-text-muted">
							No directories added
						</div>
					{:else}
						{#each excludedDirs as dir, i (i)}
							<div class="flex items-center gap-1 rounded-sm px-2 py-1 hover:bg-surface">
								<span class="min-w-0 flex-1 truncate text-sm text-text" title={dir}>
									{dir}
								</span>
								<button
									type="button"
									onclick={() => removeExcludedDir(i)}
									class="flex shrink-0 items-center justify-center rounded p-0.5 text-text-muted transition-colors hover:text-danger"
									title="Remove"
								>
									<X class="h-3.5 w-3.5" />
								</button>
							</div>
						{/each}
					{/if}
				</div>
			</div>
		</div>
	{:else if activeTab === 'items'}
		<div class="flex flex-col gap-1.5">
			<label for="excluded-items" class="text-xs font-medium text-text-muted">Excluded items</label>
			<input
				id="excluded-items"
				type="text"
				bind:value={excludedItems}
				placeholder="*/.git/*,*/node_modules/*"
				class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
			/>
			<span class="text-xs text-text-muted">Comma-separated wildcard patterns (e.g. <code>*/.git/*</code>).</span>
		</div>
	{:else if activeTab === 'settings' && activeTool === 'big-files'}
		<div class="flex flex-col gap-4">
			<div class="flex gap-4">
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="search-mode" class="text-xs font-medium text-text-muted">Search Mode</label>
					<select
						id="search-mode"
						value={toolConfig.search_mode ?? 'biggest'}
						onchange={(e) => toolConfig = { ...toolConfig, search_mode: e.currentTarget.value }}
						class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					>
						{#each SEARCH_MODES as mode (mode.value)}
							<option value={mode.value}>{mode.label}</option>
						{/each}
					</select>
				</div>
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="number-of-files" class="text-xs font-medium text-text-muted">Number of Files</label>
					<input
						id="number-of-files"
						type="number"
						min="1"
						max="10000"
						value={toolConfig.number_of_files ?? 50}
						oninput={(e) => toolConfig = { ...toolConfig, number_of_files: Number(e.currentTarget.value) }}
						class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					/>
				</div>
			</div>
		</div>
	{:else if activeTab === 'settings' && activeTool === 'similar-images'}
		<div class="flex flex-col gap-4">
			<div class="flex gap-4">
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="hash-alg" class="text-xs font-medium text-text-muted">Hash Algorithm</label>
					<select
						id="hash-alg"
						value={toolConfig.hash_alg ?? 'Gradient'}
						onchange={(e) => toolConfig = { ...toolConfig, hash_alg: e.currentTarget.value }}
						class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					>
						{#each HASH_ALGS as alg (alg)}
							<option value={alg}>{alg}</option>
						{/each}
					</select>
				</div>
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="hash-size" class="text-xs font-medium text-text-muted">Hash Size</label>
					<select
						id="hash-size"
						value={toolConfig.hash_size ?? 16}
						onchange={(e) => toolConfig = { ...toolConfig, hash_size: Number(e.currentTarget.value) }}
						class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					>
						{#each HASH_SIZES as size (size)}
							<option value={size}>{size}</option>
						{/each}
					</select>
				</div>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="resize-filter" class="text-xs font-medium text-text-muted">Resize Algorithm</label>
				<select
					id="resize-filter"
					value={toolConfig.resize_filter ?? 'Lanczos3'}
					onchange={(e) => toolConfig = { ...toolConfig, resize_filter: e.currentTarget.value }}
					class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
				>
					{#each RESIZE_FILTERS as filter (filter)}
						<option value={filter}>{filter}</option>
					{/each}
				</select>
			</div>
			<div class="flex flex-col gap-1.5">
				<label for="similarity" class="text-xs font-medium text-text-muted">Similarity</label>
				<div class="flex items-center gap-3">
					<span class="text-[10px] text-text-muted shrink-0 w-14 text-right">Very high</span>
					<input
						id="similarity"
						type="range"
						min="0"
						max="40"
						step="1"
						value={toolConfig.similarity ?? 5}
						oninput={(e) => toolConfig = { ...toolConfig, similarity: Number(e.currentTarget.value) }}
						class="flex-1 accent-accent"
					/>
					<span class="text-xs font-medium text-text w-6 text-center">{toolConfig.similarity ?? 5}</span>
					<span class="text-[10px] text-text-muted shrink-0 w-12">Minimal</span>
				</div>
			</div>
		</div>
	{/if}

	<div class="mt-4 flex items-center justify-between">
		<button
			onclick={onStartScan}
			disabled={scanState === 'running'}
			class="inline-flex items-center gap-2 rounded-md bg-accent px-5 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-accent-hover focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-surface disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{#if scanState === 'running'}
				<span class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"></span>
				Scanning…
			{:else}
				<Search class="h-4 w-4" />
				Search
			{/if}
		</button>

		<ScanActions {scanResults} {checkedFiles} {onDelete} />
	</div>
</div>
