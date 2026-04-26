<script lang="ts">
	import { Plus, X } from 'lucide-svelte';

	let {
		label,
		dirs = $bindable<string[]>(),
		onAdd
	}: {
		label: string;
		dirs: string[];
		onAdd: () => void;
	} = $props();

	function remove(index: number) {
		dirs = dirs.filter((_, i) => i !== index);
	}
</script>

<div class="flex flex-1 flex-col gap-2">
	<div class="flex items-center justify-between">
		<span class="text-xs font-medium text-text-muted">{label}</span>
		<button
			type="button"
			onclick={onAdd}
			class="flex items-center gap-1 rounded-md px-2 py-1 text-xs font-medium text-text-muted transition-colors hover:text-accent"
		>
			<Plus class="h-3.5 w-3.5" />
			Add
		</button>
	</div>
	<div
		class="flex max-h-40 flex-col gap-0.5 overflow-y-auto rounded-md border border-border bg-bg p-1 pr-0.5"
	>
		{#if dirs.length === 0}
			<div class="flex items-center justify-center py-2 text-sm text-text-muted">
				No directories added
			</div>
		{:else}
			{#each dirs as dir, i (i)}
				<div class="flex items-center gap-1 rounded-sm px-2 py-1 hover:bg-surface">
					<span class="min-w-0 flex-1 truncate text-sm text-text" title={dir}>{dir}</span>
					<button
						type="button"
						onclick={() => remove(i)}
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
