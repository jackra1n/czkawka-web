<script lang="ts">
	import { X, Folder, ChevronRight, Loader, Eye, EyeOff } from 'lucide-svelte';
	import { lastDirectory } from '$lib/stores/lastDirectory';

	interface Props {
		open: boolean;
		onClose: () => void;
		onSelect: (path: string) => void;
	}

	let { open, onClose, onSelect }: Props = $props();

	interface DirectoryEntry {
		name: string;
		path: string;
	}

	function getParentPath(path: string): string {
		if (path === '/') return '/';
		const parts = path.split('/').filter(Boolean);
		if (parts.length <= 1) return '/';
		parts.pop();
		return '/' + parts.join('/');
	}

	let fetchPath = $state('');
	let displayPath = $state('');
	let directories = $state<DirectoryEntry[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let showHidden = $state(false);

	async function fetchDirs(path: string, includeHidden: boolean) {
		loading = true;
		error = null;
		try {
			const res = await fetch(`/api/directories?path=${encodeURIComponent(path)}&hidden=${includeHidden}`);
			if (res.ok) {
				const data = await res.json();
				directories = data.directories;
				displayPath = data.path || path;
			} else {
				error = 'Failed to load directories';
				directories = [];
			}
		} catch {
			error = 'Failed to connect to server';
			directories = [];
		} finally {
			loading = false;
		}
	}

	function getBreadcrumbs(path: string): { name: string; path: string }[] {
		if (path === '/') return [{ name: '/', path: '/' }];
		const parts = path.split('/').filter(Boolean);
		const crumbs = [{ name: '/', path: '/' }];
		let build = '';
		for (const part of parts) {
			build += '/' + part;
			crumbs.push({ name: part, path: build });
		}
		return crumbs;
	}

	function selectFolder() {
		lastDirectory.set(getParentPath(displayPath));
		onSelect(displayPath);
		onClose();
	}

	$effect(() => {
		if (open) {
			const stored = $lastDirectory;
			fetchPath = stored && stored !== '~' && stored !== '~/' ? stored : '~';
		}
	});

	$effect(() => {
		if (open && fetchPath) {
			fetchDirs(fetchPath, showHidden);
		}
	});

	let breadcrumbs = $derived(getBreadcrumbs(displayPath));
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
		onclick={onClose}
		onkeydown={(e) => e.key === 'Escape' && onClose()}
		role="presentation"
	>
		<div
			class="flex h-120 w-130 flex-col rounded-xl border border-border bg-surface shadow-2xl"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="flex items-center justify-between border-b border-border px-5 py-4">
				<h2 class="text-sm font-semibold">Select folder</h2>
				<button onclick={onClose} class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text">
					<X class="h-4 w-4" />
				</button>
			</div>

			<div class="flex items-center gap-1 border-b border-border px-5 py-2.5 text-xs text-text-muted">
				{#each breadcrumbs as crumb, i (crumb.path)}
					<button class="transition-colors hover:text-accent" onclick={() => { fetchPath = crumb.path; }}>
						{crumb.name}
					</button>
					{#if i < breadcrumbs.length - 1}
						<ChevronRight class="h-3 w-3 shrink-0" />
					{/if}
				{/each}
			</div>

			<div class="flex-1 overflow-y-auto px-2 py-1">
				{#if loading}
					<div class="flex items-center justify-center py-8">
						<Loader class="h-5 w-5 animate-spin text-text-muted" />
					</div>
				{:else if error}
					<p class="px-3 py-8 text-center text-sm text-red-500">{error}</p>
				{:else}
					{#if displayPath !== '/'}
						<button
							class="flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left text-sm text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
							onclick={() => { fetchPath = getParentPath(displayPath); }}
						>
							<Folder class="h-4 w-4 opacity-50" />
							<span>..</span>
						</button>
					{/if}
					{#each directories as folder (folder.path)}
						<button
							class="flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left text-sm text-text transition-colors hover:bg-surface-raised"
							onclick={() => { fetchPath = folder.path; }}
						>
							<Folder class="h-4 w-4 text-accent" />
							<span>{folder.name}</span>
						</button>
					{/each}
					{#if directories.length === 0}
						<p class="px-3 py-8 text-center text-sm text-text-muted">No folders available.</p>
					{/if}
				{/if}
			</div>

			<div class="flex items-center justify-between gap-3 border-t border-border px-5 py-4">
				<button
					onclick={() => showHidden = !showHidden}
					class="flex items-center gap-1.5 rounded border border-border px-3 py-2 text-xs font-medium transition-colors {showHidden ? 'bg-surface-raised text-accent' : 'text-text-muted hover:bg-surface-raised hover:text-text'}"
					title={showHidden ? 'Hide hidden folders' : 'Show hidden folders'}
				>
					{#if showHidden}
						<Eye class="h-3.5 w-3.5" />
					{:else}
						<EyeOff class="h-3.5 w-3.5" />
					{/if}
					<span>Hidden</span>
				</button>
				<div class="flex items-center gap-3">
					<button onclick={onClose} class="rounded-md border border-border px-4 py-2 text-sm font-medium text-text transition-colors hover:bg-surface-raised">
						Cancel
					</button>
					<button onclick={selectFolder} class="rounded-md bg-accent px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-accent-hover">
						Select "{breadcrumbs.at(-1)?.name ?? ''}"
					</button>
				</div>
			</div>
		</div>
	</div>
{/if}
