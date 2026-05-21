<script lang="ts">
	import { Plus, X, Eye, EyeOff, RotateCcw } from 'lucide-svelte';
	import Tooltip from './ui/Tooltip.svelte';

	let {
		label,
		dirs = $bindable<string[]>(),
		onAdd,
		defaultDirs = [],
	}: {
		label: string;
		dirs: string[];
		onAdd: () => void;
		defaultDirs?: string[];
	} = $props();

	let showDefaults = $state(false);

	let visibleCount = $derived(showDefaults ? dirs.length : dirs.filter((d) => !defaultDirs.includes(d)).length);

	function remove(value: string) {
		dirs = dirs.filter((d) => d !== value);
	}

	const hasChanges = $derived.by(() => {
		if (defaultDirs.length === 0) return false;
		if (dirs.length !== defaultDirs.length) return true;
		const defaultSet = new Set(defaultDirs);
		return !dirs.every((d) => defaultSet.has(d));
	});

	function resetToDefaults() {
		dirs = [...defaultDirs];
	}
</script>

<div class="flex flex-1 flex-col">
	<div class="flex items-center justify-between">
		<span class="text-xs font-medium text-text-muted">{label}</span>
		<div class="flex items-center gap-1">
			{#if defaultDirs.length > 0}
				{#if hasChanges}
					<Tooltip content="Reset to default directories">
						<button
							type="button"
							onclick={resetToDefaults}
							class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
						>
							<RotateCcw class="h-3.5 w-3.5" />
							Reset
						</button>
					</Tooltip>
				{/if}
				<Tooltip content={showDefaults ? 'Hide default directories' : 'Show default directories'}>
					<button
						type="button"
						onclick={() => (showDefaults = !showDefaults)}
						class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
					>
						{#if showDefaults}
							<EyeOff class="h-3.5 w-3.5" />
						{:else}
							<Eye class="h-3.5 w-3.5" />
						{/if}
						Defaults
					</button>
				</Tooltip>
			{/if}
			<button
				type="button"
				onclick={onAdd}
				class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
			>
				<Plus class="h-3.5 w-3.5" />
				Add
			</button>
		</div>
	</div>
	<div class="flex max-h-40 flex-col gap-0.5 overflow-y-auto rounded-md border border-border bg-bg p-1 pr-0.5">
		{#if visibleCount === 0}
			<div class="flex items-center justify-center py-2 text-sm text-text-muted">
				{dirs.length === 0 ? 'No directories added' : 'Default directories hidden'}
			</div>
		{:else}
			{#each dirs as dir (dir)}
				{#if showDefaults || !defaultDirs.includes(dir)}
					<div class="flex items-center gap-1 rounded-sm px-2 py-1 hover:bg-surface">
						<Tooltip class="flex min-w-0 flex-1" content={dir}>
							<span class="min-w-0 flex-1 truncate text-sm text-text">{dir}</span>
						</Tooltip>
						<Tooltip class="inline-flex shrink-0" content="Remove">
							<button
								type="button"
								onclick={() => remove(dir)}
								class="flex shrink-0 items-center justify-center rounded p-0.5 text-text-muted transition-colors hover:text-danger"
							>
								<X class="h-3.5 w-3.5" />
							</button>
						</Tooltip>
					</div>
				{/if}
			{/each}
		{/if}
	</div>
</div>
