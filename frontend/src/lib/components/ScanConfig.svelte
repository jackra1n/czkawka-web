<script lang="ts">
	import { Search } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ToolConfig } from '$lib/api';
	import DirectoryList from './DirectoryList.svelte';
	import ScanActions from './ScanActions.svelte';
	import ToolSettings from './ToolSettings.svelte';

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
		onDelete,
		onFix
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
		onFix: () => void;
	} = $props();

	let activeTab = $state<'directories' | 'items' | 'settings'>('directories');

	const SETTINGS_TOOLS = new Set([
		'similar-images',
		'similar-videos',
		'big-files',
		'same-music',
		'broken-files',
		'bad-extensions',
		'bad-names'
	]);

	$effect(() => {
		if (activeTab === 'settings' && !SETTINGS_TOOLS.has(activeTool)) {
			activeTab = 'directories';
		}
	});
</script>

<div class="shrink-0 border-b border-border bg-surface p-4">
	<div class="mb-3 flex border-b border-border">
		{#each [{ k: 'directories', l: 'Directories' }, { k: 'items', l: 'Items' }, { k: 'settings', l: 'Settings' }] as t (t.k)}
			{#if t.k !== 'settings' || SETTINGS_TOOLS.has(activeTool)}
				<button
					type="button"
					onclick={() => (activeTab = t.k as typeof activeTab)}
					class="relative px-3 py-2 text-xs font-medium transition-colors {activeTab === t.k
						? 'text-accent'
						: 'text-text-muted hover:text-text'}"
				>
					{t.l}
					{#if activeTab === t.k}
						<span class="absolute right-0 bottom-0 left-0 h-0.5 rounded-t-sm bg-accent"></span>
					{/if}
				</button>
			{/if}
		{/each}
	</div>

	{#if activeTab === 'directories'}
		<div class="flex gap-4">
			<DirectoryList
				label="Included directories"
				bind:dirs={includedDirs}
				onAdd={() => onAddDir('include')}
			/>
			<DirectoryList
				label="Excluded directories"
				bind:dirs={excludedDirs}
				onAdd={() => onAddDir('exclude')}
			/>
		</div>
	{:else if activeTab === 'items'}
		<div class="flex flex-col gap-1.5">
			<label for="excluded-items" class="text-xs font-medium text-text-muted">Excluded items</label>
			<input
				id="excluded-items"
				type="text"
				bind:value={excludedItems}
				placeholder="*/.git/*,*/node_modules/*"
				class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:ring-1 focus:ring-accent focus:outline-none"
			/>
			<span class="text-xs text-text-muted"
				>Comma-separated wildcard patterns (e.g. <code>*/.git/*</code>).</span
			>
		</div>
	{:else if activeTab === 'settings'}
		<ToolSettings {activeTool} bind:toolConfig />
	{/if}

	<div class="mt-4 flex items-center justify-between">
		<button
			onclick={onStartScan}
			disabled={scanState === 'running'}
			class="inline-flex items-center gap-2 rounded-md bg-accent px-5 py-2.5 text-sm font-semibold text-white transition-colors hover:bg-accent-hover focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-surface focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
		>
			{#if scanState === 'running'}
				<span
					class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"
				></span>
				Scanning…
			{:else}
				<Search class="h-4 w-4" />
				Search
			{/if}
		</button>

		<ScanActions {scanResults} {checkedFiles} {activeTool} {onDelete} {onFix} />
	</div>
</div>
