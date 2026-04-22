<script lang="ts">
	import { ChevronDown, Trash2 } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, DuplicateFile } from '$lib/api';
	import ConfirmModal from './ConfirmModal.svelte';

	let {
		scanResults,
		checkedFiles,
		onDelete
	}: {
		scanResults: ScanResults | null;
		checkedFiles: SvelteSet<string>;
		onDelete: () => void;
	} = $props();

	let selectOpen = $state(false);
	let showDeleteConfirm = $state(false);

	const hasResults = $derived(!!scanResults && scanResults.groups.length > 0);

	function closeDropdown() {
		selectOpen = false;
	}

	$effect(() => {
		if (selectOpen) {
			const handler = (e: MouseEvent) => {
				if (!(e.target as HTMLElement).closest('.select-dropdown')) {
					selectOpen = false;
				}
			};
			const id = setTimeout(() => window.addEventListener('click', handler), 0);
			return () => {
				clearTimeout(id);
				window.removeEventListener('click', handler);
			};
		}
	});

	function selectAll() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			for (const file of group.files) {
				checkedFiles.add(file.path);
			}
		}
		closeDropdown();
	}

	function unselectAll() {
		checkedFiles.clear();
		closeDropdown();
	}

	function invertSelection() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			for (const file of group.files) {
				if (checkedFiles.has(file.path)) {
					checkedFiles.delete(file.path);
				} else {
					checkedFiles.add(file.path);
				}
			}
		}
		closeDropdown();
	}

	function selectAllExceptOldest() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let oldest: DuplicateFile | null = null;
			for (const file of group.files) {
				if (!oldest || (file.modified_date ?? 0) < (oldest.modified_date ?? 0)) {
					oldest = file;
				}
			}
			for (const file of group.files) {
				if (file !== oldest) {
					checkedFiles.add(file.path);
				} else {
					checkedFiles.delete(file.path);
				}
			}
		}
		closeDropdown();
	}

	function selectAllExceptNewest() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let newest: DuplicateFile | null = null;
			for (const file of group.files) {
				if (!newest || (file.modified_date ?? 0) > (newest.modified_date ?? 0)) {
					newest = file;
				}
			}
			for (const file of group.files) {
				if (file !== newest) {
					checkedFiles.add(file.path);
				} else {
					checkedFiles.delete(file.path);
				}
			}
		}
		closeDropdown();
	}

	function selectAllExceptShortestPath() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let shortest = group.files[0];
			for (const file of group.files) {
				if (file.path.length < shortest.path.length) {
					shortest = file;
				}
			}
			for (const file of group.files) {
				if (file !== shortest) {
					checkedFiles.add(file.path);
				} else {
					checkedFiles.delete(file.path);
				}
			}
		}
		closeDropdown();
	}

	function selectAllExceptLongestPath() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let longest = group.files[0];
			for (const file of group.files) {
				if (file.path.length > longest.path.length) {
					longest = file;
				}
			}
			for (const file of group.files) {
				if (file !== longest) {
					checkedFiles.add(file.path);
				} else {
					checkedFiles.delete(file.path);
				}
			}
		}
		closeDropdown();
	}

	type SelectOption =
		| { type: 'item'; label: string; action: () => void }
		| { type: 'separator' };

	const selectOptions: SelectOption[] = [
		{ type: 'item', label: 'Select all', action: selectAll },
		{ type: 'item', label: 'Unselect all', action: unselectAll },
		{ type: 'item', label: 'Invert selection', action: invertSelection },
		{ type: 'separator' },
		{ type: 'item', label: 'Select all except oldest', action: selectAllExceptOldest },
		{ type: 'item', label: 'Select all except newest', action: selectAllExceptNewest },
		{ type: 'separator' },
		{ type: 'item', label: 'Select all except shortest path', action: selectAllExceptShortestPath },
		{ type: 'item', label: 'Select all except longest path', action: selectAllExceptLongestPath }
	];
</script>

<div class="flex items-center gap-2">
	<div class="select-dropdown relative">
		<button
			type="button"
			onclick={() => (selectOpen = !selectOpen)}
			disabled={!hasResults}
			class="inline-flex items-center gap-1.5 rounded-md border border-border bg-surface-raised px-3 py-2 text-sm font-medium text-text transition-colors hover:bg-surface disabled:opacity-50 disabled:cursor-not-allowed"
		>
			Select
			<ChevronDown class="h-4 w-4 text-text-muted" />
		</button>
		{#if selectOpen}
			<div class="absolute right-0 mt-1 w-60 overflow-hidden rounded-md border border-border bg-surface shadow-lg z-20">
				{#each selectOptions as option, i (option.type === 'separator' ? `sep-${i}` : option.label)}
					{#if option.type === 'separator'}
						<div class="my-1 h-px bg-border"></div>
					{:else}
						<button
							type="button"
							onclick={option.action}
							class="w-full px-3 py-2 text-left text-sm text-text transition-colors hover:bg-surface-raised"
						>
							{option.label}
						</button>
					{/if}
				{/each}
			</div>
		{/if}
	</div>

	<button
		type="button"
		onclick={() => (showDeleteConfirm = true)}
		disabled={checkedFiles.size === 0}
		class="inline-flex items-center gap-1.5 rounded-md border border-danger/30 bg-danger/10 px-3 py-2 text-sm font-medium text-danger transition-colors hover:bg-danger/20 disabled:opacity-50 disabled:cursor-not-allowed"
	>
		<Trash2 class="h-4 w-4" />
		Delete
	</button>
</div>

<ConfirmModal
	open={showDeleteConfirm}
	title="Delete Files"
	message={`Are you sure you want to delete ${checkedFiles.size} selected file${checkedFiles.size === 1 ? '' : 's'}? This action cannot be undone.`}
	confirmText="Delete"
	cancelText="Cancel"
	onConfirm={() => {
		showDeleteConfirm = false;
		onDelete();
	}}
	onCancel={() => (showDeleteConfirm = false)}
/>
