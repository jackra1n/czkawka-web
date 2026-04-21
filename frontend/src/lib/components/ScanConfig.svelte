<script lang="ts">
	import { Folder, Plus, Minus, Search } from 'lucide-svelte';

	let {
		includedDirs = $bindable<string[]>(),
		excludedDirs = $bindable<string[]>(),
		scanState,
		onStartScan,
		onOpenModal
	}: {
		includedDirs: string[];
		excludedDirs: string[];
		scanState: 'idle' | 'running' | 'completed' | 'error';
		onStartScan: () => void;
		onOpenModal: (target: 'include' | 'exclude', index: number) => void;
	} = $props();

	function addIncludedDir() {
		if (includedDirs.length > 0 && includedDirs[includedDirs.length - 1].trim() === '') return;
		includedDirs = [...includedDirs, ''];
	}

	function removeIncludedDir(index: number) {
		includedDirs = includedDirs.filter((_, i) => i !== index);
		if (includedDirs.length === 0) includedDirs = [''];
	}

	function updateIncludedDir(index: number, value: string) {
		includedDirs[index] = value;
	}

	function addExcludedDir() {
		if (excludedDirs.length > 0 && excludedDirs[excludedDirs.length - 1].trim() === '') return;
		excludedDirs = [...excludedDirs, ''];
	}

	function removeExcludedDir(index: number) {
		excludedDirs = excludedDirs.filter((_, i) => i !== index);
		if (excludedDirs.length === 0) excludedDirs = [''];
	}

	function updateExcludedDir(index: number, value: string) {
		excludedDirs[index] = value;
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
			<div class="flex max-h-48 flex-col gap-1.5 overflow-y-auto pr-1">
				{#each includedDirs as dir, i (i)}
					<div class="flex gap-2">
						<input
							type="text"
							value={dir}
							oninput={(e) => updateIncludedDir(i, e.currentTarget.value)}
							placeholder="/home/user/Downloads"
							class="min-w-0 flex-1 rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
						/>
						<button
							type="button"
							onclick={() => onOpenModal('include', i)}
							class="flex shrink-0 items-center justify-center rounded-md border border-border bg-bg px-2.5 py-2 text-text-muted transition-colors hover:border-accent hover:text-accent"
							title="Browse"
						>
							<Folder class="h-4 w-4" />
						</button>
						<button
							type="button"
							onclick={() => removeIncludedDir(i)}
							class="flex shrink-0 items-center justify-center rounded-md border border-border bg-bg px-2.5 py-2 text-text-muted transition-colors hover:border-danger hover:text-danger"
							title="Remove"
						>
							<Minus class="h-4 w-4" />
						</button>
					</div>
				{/each}
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
			<div class="flex max-h-48 flex-col gap-1.5 overflow-y-auto pr-1">
				{#each excludedDirs as dir, i (i)}
					<div class="flex gap-2">
						<input
							type="text"
							value={dir}
							oninput={(e) => updateExcludedDir(i, e.currentTarget.value)}
							placeholder="/home/user/Downloads/temp"
							class="min-w-0 flex-1 rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
						/>
						<button
							type="button"
							onclick={() => onOpenModal('exclude', i)}
							class="flex shrink-0 items-center justify-center rounded-md border border-border bg-bg px-2.5 py-2 text-text-muted transition-colors hover:border-accent hover:text-accent"
							title="Browse"
						>
							<Folder class="h-4 w-4" />
						</button>
						<button
							type="button"
							onclick={() => removeExcludedDir(i)}
							class="flex shrink-0 items-center justify-center rounded-md border border-border bg-bg px-2.5 py-2 text-text-muted transition-colors hover:border-danger hover:text-danger"
							title="Remove"
						>
							<Minus class="h-4 w-4" />
						</button>
					</div>
				{/each}
			</div>
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
