<script lang="ts">
	import { Search, ChevronDown, ChevronUp, RotateCcw } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ToolConfig } from '$lib/api';
	import DirectoryList from './DirectoryList.svelte';
	import ScanActions from './ScanActions.svelte';
	import ToolSettings from './ToolSettings.svelte';
	import Tooltip from './ui/Tooltip.svelte';
	import { DEFAULT_TOOL_CONFIGS } from '$lib/defaults';

	let {
		includedDirs = $bindable<string[]>(),
		excludedDirs = $bindable<string[]>(),
		excludedItems = $bindable<string>(),
		activeTool = $bindable<string>(),
		toolConfig = $bindable<ToolConfig>({}),
		defaultExcludedDirs = [],
		scanState,
		scanResults,
		checkedFiles,
		onStartScan,
		onCancelScan,
		onAddDir,
		onDelete,
		onFix,
	}: {
		includedDirs: string[];
		excludedDirs: string[];
		excludedItems: string;
		activeTool: string;
		toolConfig: ToolConfig;
		defaultExcludedDirs?: string[];
		scanState: 'idle' | 'running' | 'cancelling' | 'completed' | 'error';
		scanResults: ScanResults | null;
		checkedFiles: SvelteSet<string>;
		onStartScan: () => void;
		onCancelScan: () => void;
		onAddDir: (target: 'include' | 'exclude') => void;
		onDelete: () => void;
		onFix: () => void;
	} = $props();

	let activeTab = $state<'directories' | 'items' | 'settings'>('directories');
	let collapsed = $state(false);

	const SETTINGS_TOOLS = new Set([
		'similar-images',
		'similar-videos',
		'big-files',
		'same-music',
		'broken-files',
		'bad-extensions',
		'bad-names',
	]);

	$effect(() => {
		if (activeTab === 'settings' && !SETTINGS_TOOLS.has(activeTool)) {
			activeTab = 'directories';
		}
	});

	$effect(() => {
		if (scanState === 'running') {
			collapsed = true;
		}
	});

	const hasChanges = $derived.by(() => {
		const def = DEFAULT_TOOL_CONFIGS[activeTool];
		if (!def) return false;
		for (const key of Object.keys(def) as Array<keyof ToolConfig>) {
			const val = toolConfig[key];
			const defVal = def[key];
			if (val !== undefined && val !== defVal) {
				return true;
			}
		}
		return false;
	});

	function resetSettings() {
		const def = DEFAULT_TOOL_CONFIGS[activeTool];
		if (def) {
			toolConfig = { ...toolConfig, ...def };
		}
	}
</script>

<div class="shrink-0 border-b border-border bg-surface p-4">
	<button
		type="button"
		onclick={() => (collapsed = !collapsed)}
		class="flex w-full cursor-pointer items-center justify-between text-sm font-semibold text-text transition-colors hover:text-text-muted"
	>
		<span>Configuration</span>
		{#if collapsed}
			<ChevronDown class="h-4 w-4" />
		{:else}
			<ChevronUp class="h-4 w-4" />
		{/if}
	</button>

	<div
		class="grid transition-all duration-300 ease-in-out"
		class:grid-rows-[0fr]={collapsed}
		class:grid-rows-[1fr]={!collapsed}
	>
		<div class="overflow-hidden">
			<div class="mb-3 flex items-center justify-between border-b border-border pt-3">
				<div class="flex">
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

				{#if activeTab === 'settings' && hasChanges}
					<Tooltip content="Reset settings to defaults">
						<button
							type="button"
							onclick={resetSettings}
							class="mb-1 flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
						>
							<RotateCcw class="h-3.5 w-3.5" />
							Reset settings
						</button>
					</Tooltip>
				{/if}
			</div>

			{#if activeTab === 'directories'}
				<div class="flex gap-4">
					<DirectoryList label="Included directories" bind:dirs={includedDirs} onAdd={() => onAddDir('include')} />
					<DirectoryList
						label="Excluded directories"
						bind:dirs={excludedDirs}
						onAdd={() => onAddDir('exclude')}
						defaultDirs={defaultExcludedDirs}
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
					<span class="text-xs text-text-muted">Comma-separated wildcard patterns (e.g. <code>*/.git/*</code>).</span>
				</div>
			{:else if activeTab === 'settings'}
				<ToolSettings {activeTool} bind:toolConfig />
			{/if}
		</div>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div class="flex items-center gap-2">
			<button
				onclick={onStartScan}
				disabled={scanState === 'running' || scanState === 'cancelling'}
				class="inline-flex items-center gap-1.5 rounded-md bg-accent px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-accent-hover focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-surface focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
			>
				{#if scanState === 'running'}
					<span class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"></span>
					Scanning…
				{:else if scanState === 'cancelling'}
					<span class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"></span>
					Cancelling…
				{:else}
					<Search class="h-4 w-4" />
					Search
				{/if}
			</button>

			{#if scanState === 'running'}
				<button
					type="button"
					onclick={onCancelScan}
					class="inline-flex items-center gap-1.5 rounded-md border border-danger/30 bg-danger/10 px-3 py-1.5 text-sm font-medium text-danger transition-colors hover:bg-danger/20 focus:ring-2 focus:ring-danger focus:ring-offset-2 focus:ring-offset-surface focus:outline-none"
				>
					Cancel
				</button>
			{/if}
		</div>

		<ScanActions {scanResults} {checkedFiles} {activeTool} {onDelete} {onFix} />
	</div>
</div>
