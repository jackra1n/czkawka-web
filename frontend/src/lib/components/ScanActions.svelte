<script lang="ts">
	import { ChevronDown, Trash2, Pencil, Sparkles, Link, Link2 } from 'lucide-svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import type { ScanResults, ScannedFile } from '$lib/api';
	import ConfirmModal from './ConfirmModal.svelte';

	let {
		scanResults,
		checkedFiles,
		activeTool,
		onDelete,
		onFix,
		onLink,
	}: {
		scanResults: ScanResults | null;
		checkedFiles: SvelteSet<string>;
		activeTool: string;
		onDelete: () => void;
		onFix: () => void;
		onLink: (type: 'hard' | 'soft') => void;
	} = $props();

	let selectOpen = $state(false);
	let showDeleteConfirm = $state(false);
	let showFixConfirm = $state(false);
	let showLinkConfirm = $state<'hard' | 'soft' | null>(null);

	const hasResults = $derived(!!scanResults && scanResults.groups.length > 0);
	const hasChecked = $derived(checkedFiles.size > 0);

	const showFixButton = $derived(
		activeTool === 'exif-remover' || activeTool === 'bad-names' || activeTool === 'bad-extensions',
	);
	const showLinkButtons = $derived(
		activeTool === 'duplicates' ||
			activeTool === 'same-music' ||
			activeTool === 'similar-images' ||
			activeTool === 'similar-videos',
	);
	const fixLabel = $derived(activeTool === 'exif-remover' ? 'Clean' : 'Rename');
	const FixIcon = $derived(activeTool === 'exif-remover' ? Sparkles : Pencil);
	const fixConfirmTitle = $derived(activeTool === 'exif-remover' ? 'Clean EXIF Data' : 'Rename Files');
	const fixConfirmMessage = $derived(
		activeTool === 'exif-remover'
			? `Are you sure you want to remove EXIF data from ${checkedFiles.size} selected file${checkedFiles.size === 1 ? '' : 's'}?`
			: `Are you sure you want to rename ${checkedFiles.size} selected file${checkedFiles.size === 1 ? '' : 's'}?`,
	);

	function closeDropdown() {
		selectOpen = false;
	}

	function getPixelCount(dimensions?: string): number {
		if (!dimensions) return 0;
		const match = dimensions.match(/^(\d+)x(\d+)$/);
		if (!match) return 0;
		return parseInt(match[1], 10) * parseInt(match[2], 10);
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
			let oldest: ScannedFile | null = null;
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
			let newest: ScannedFile | null = null;
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

	function selectAllExceptBiggest() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let biggest: ScannedFile | null = null;
			for (const file of group.files) {
				if (!biggest) {
					biggest = file;
					continue;
				}
				const filePixels = getPixelCount(file.dimensions);
				const biggestPixels = getPixelCount(biggest.dimensions);
				if (filePixels > biggestPixels) {
					biggest = file;
				} else if (filePixels === biggestPixels) {
					const fileSize = file.size ?? 0;
					const biggestSize = biggest.size ?? 0;
					if (fileSize > biggestSize) {
						biggest = file;
					}
				}
			}
			for (const file of group.files) {
				if (file !== biggest) {
					checkedFiles.add(file.path);
				} else {
					checkedFiles.delete(file.path);
				}
			}
		}
		closeDropdown();
	}

	function selectAllExceptSmallest() {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			let smallest: ScannedFile | null = null;
			for (const file of group.files) {
				if (!smallest) {
					smallest = file;
					continue;
				}
				const filePixels = getPixelCount(file.dimensions);
				const smallestPixels = getPixelCount(smallest.dimensions);
				if (filePixels < smallestPixels) {
					smallest = file;
				} else if (filePixels === smallestPixels) {
					const fileSize = file.size ?? 0;
					const smallestSize = smallest.size ?? 0;
					if (fileSize < smallestSize) {
						smallest = file;
					}
				}
			}
			for (const file of group.files) {
				if (file !== smallest) {
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

	type SelectOption = { type: 'item'; label: string; action: () => void } | { type: 'separator' };

	const selectOptions: SelectOption[] = [
		{ type: 'item', label: 'Select all', action: selectAll },
		{ type: 'item', label: 'Unselect all', action: unselectAll },
		{ type: 'item', label: 'Invert selection', action: invertSelection },
		{ type: 'separator' },
		{ type: 'item', label: 'Select all except oldest', action: selectAllExceptOldest },
		{ type: 'item', label: 'Select all except newest', action: selectAllExceptNewest },
		{ type: 'separator' },
		{ type: 'item', label: 'Select all except biggest', action: selectAllExceptBiggest },
		{ type: 'item', label: 'Select all except smallest', action: selectAllExceptSmallest },
		{ type: 'separator' },
		{ type: 'item', label: 'Select all except shortest path', action: selectAllExceptShortestPath },
		{ type: 'item', label: 'Select all except longest path', action: selectAllExceptLongestPath },
	];
</script>

<div class="flex items-center gap-2">
	<div class="select-dropdown relative">
		<button
			type="button"
			onclick={() => (selectOpen = !selectOpen)}
			disabled={!hasResults}
			class="inline-flex items-center gap-1.5 rounded-md border border-border bg-surface-raised px-3 py-1.5 text-sm font-medium text-text transition-colors hover:bg-surface disabled:cursor-not-allowed disabled:opacity-50"
		>
			Select
			<ChevronDown class="h-4 w-4 text-text-muted" />
		</button>
		{#if selectOpen}
			<div class="absolute right-0 z-20 mt-1 w-60 overflow-hidden rounded-md border border-border bg-surface shadow-lg">
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

	{#if showFixButton}
		<button
			type="button"
			onclick={() => (showFixConfirm = true)}
			disabled={!hasChecked}
			class="inline-flex items-center gap-1.5 rounded-md border border-accent/30 bg-accent/10 px-3 py-1.5 text-sm font-medium text-accent transition-colors hover:bg-accent/20 disabled:cursor-not-allowed disabled:opacity-50"
		>
			<FixIcon class="h-4 w-4" />
			{fixLabel}
		</button>
	{/if}

	{#if showLinkButtons}
		<button
			type="button"
			onclick={() => (showLinkConfirm = 'hard')}
			disabled={!hasChecked}
			class="inline-flex items-center gap-1.5 rounded-md border border-accent/30 bg-accent/10 px-3 py-1.5 text-sm font-medium text-accent transition-colors hover:bg-accent/20 disabled:cursor-not-allowed disabled:opacity-50"
		>
			<Link2 class="h-4 w-4" />
			Hardlink
		</button>
		<button
			type="button"
			onclick={() => (showLinkConfirm = 'soft')}
			disabled={!hasChecked}
			class="inline-flex items-center gap-1.5 rounded-md border border-accent/30 bg-accent/10 px-3 py-1.5 text-sm font-medium text-accent transition-colors hover:bg-accent/20 disabled:cursor-not-allowed disabled:opacity-50"
		>
			<Link class="h-4 w-4" />
			Softlink
		</button>
	{/if}

	<button
		type="button"
		onclick={() => (showDeleteConfirm = true)}
		disabled={!hasChecked}
		class="inline-flex items-center gap-1.5 rounded-md border border-danger/30 bg-danger/10 px-3 py-1.5 text-sm font-medium text-danger transition-colors hover:bg-danger/20 disabled:cursor-not-allowed disabled:opacity-50"
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

<ConfirmModal
	open={showFixConfirm}
	title={fixConfirmTitle}
	message={fixConfirmMessage}
	confirmText={fixLabel}
	cancelText="Cancel"
	onConfirm={() => {
		showFixConfirm = false;
		onFix();
	}}
	onCancel={() => (showFixConfirm = false)}
/>

<ConfirmModal
	open={showLinkConfirm !== null}
	title={showLinkConfirm === 'hard' ? 'Create Hardlinks' : 'Create Softlinks'}
	message={showLinkConfirm === 'hard'
		? `Are you sure you want to replace the selected ${checkedFiles.size} checked file${checkedFiles.size === 1 ? '' : 's'} with hardlinks?`
		: `Are you sure you want to replace the selected ${checkedFiles.size} checked file${checkedFiles.size === 1 ? '' : 's'} with softlinks (symlinks)?`}
	confirmText={showLinkConfirm === 'hard' ? 'Hardlink' : 'Softlink'}
	cancelText="Cancel"
	onConfirm={() => {
		const type = showLinkConfirm;
		showLinkConfirm = null;
		if (type) onLink(type);
	}}
	onCancel={() => (showLinkConfirm = null)}
/>
