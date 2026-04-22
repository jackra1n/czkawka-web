<script lang="ts">
	import { Plus, X, Search } from 'lucide-svelte';

	let {
		includedDirs = $bindable<string[]>(),
		excludedDirs = $bindable<string[]>(),
		excludedItems = $bindable<string>(),
		scanState,
		onStartScan,
		onAddDir
	}: {
		includedDirs: string[];
		excludedDirs: string[];
		excludedItems: string;
		scanState: 'idle' | 'running' | 'completed' | 'error';
		onStartScan: () => void;
		onAddDir: (target: 'include' | 'exclude') => void;
	} = $props();

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
</script>

<div class="shrink-0 border-b border-border bg-surface p-4">
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
	<div class="mt-4">
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
	</div>
	<div class="mt-4">
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
	</div>
</div>
