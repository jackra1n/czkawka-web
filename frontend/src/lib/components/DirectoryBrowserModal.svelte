<script lang="ts">
	import { X, Folder, ChevronRight, Loader } from 'lucide-svelte';

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

	let currentPath = $state('/');
	let directories = $state<DirectoryEntry[]>([]);
	let loading = $state(false);
	let error = $state<string | null>(null);

	async function fetchDirectories(path: string) {
		loading = true;
		error = null;
		try {
			const res = await fetch(`/api/directories?path=${encodeURIComponent(path)}`);
			if (res.ok) {
				const data = await res.json();
				directories = data.directories;
			} else {
				error = 'Failed to load directories';
				directories = [];
			}
		} catch (e) {
			error = 'Failed to connect to server';
			directories = [];
		} finally {
			loading = false;
		}
	}

	function getBreadcrumbs(path: string): { name: string; path: string }[] {
		if (path === '/') return [{ name: 'root', path: '/' }];
		const parts = path.split('/').filter(Boolean);
		const crumbs = [{ name: 'root', path: '/' }];
		let build = '';
		for (const part of parts) {
			build += '/' + part;
			crumbs.push({ name: part, path: build });
		}
		return crumbs;
	}

	function navigateTo(path: string) {
		currentPath = path;
		fetchDirectories(path);
	}

	function selectCurrent() {
		onSelect(currentPath);
		onClose();
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onClose();
	}

	$effect(() => {
		if (open && currentPath === '/') {
			fetchDirectories('/');
		}
	});

	let breadcrumbs = $derived(getBreadcrumbs(currentPath));
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
		onclick={onClose}
		onkeydown={handleKeydown}
		role="presentation"
	>
		<div
			class="flex h-[480px] w-[520px] flex-col rounded-xl border border-border bg-surface shadow-2xl"
			onclick={(e) => e.stopPropagation()}
			onkeydown={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<!-- Header -->
			<div class="flex items-center justify-between border-b border-border px-5 py-4">
				<h2 class="text-sm font-semibold">Select folder</h2>
				<button
					onclick={onClose}
					class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
				>
					<X class="h-4 w-4" />
				</button>
			</div>

			<!-- Breadcrumb -->
			<div class="flex items-center gap-1 border-b border-border px-5 py-2.5 text-xs text-text-muted">
				{#each breadcrumbs as crumb, i}
					<button
						class="transition-colors hover:text-accent"
						onclick={() => navigateTo(crumb.path)}
					>
						{crumb.name}
					</button>
					{#if i < breadcrumbs.length - 1}
						<ChevronRight class="h-3 w-3 shrink-0" />
					{/if}
				{/each}
			</div>

			<!-- Folder list -->
			<div class="flex-1 overflow-y-auto px-2 py-1">
				{#if loading}
					<div class="flex items-center justify-center py-8">
						<Loader class="h-5 w-5 animate-spin text-text-muted" />
					</div>
				{:else if error}
					<p class="px-3 py-8 text-center text-sm text-red-500">{error}</p>
				{:else}
					{#if currentPath !== '/'}
						<button
							class="flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left text-sm text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
							onclick={() => {
								const parent = currentPath.split('/').slice(0, -1).join('/') || '/';
								navigateTo(parent);
							}}
						>
							<Folder class="h-4 w-4 opacity-50" />
							<span>..</span>
						</button>
					{/if}
					{#each directories as folder}
						<button
							class="flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left text-sm text-text transition-colors hover:bg-surface-raised"
							onclick={() => navigateTo(folder.path)}
						>
							<Folder class="h-4 w-4 text-accent" />
							<span>{folder.name}</span>
						</button>
					{/each}
					{#if directories.length === 0 && currentPath === '/'}
						<p class="px-3 py-8 text-center text-sm text-text-muted">No folders available.</p>
					{/if}
				{/if}
			</div>

			<!-- Footer -->
			<div class="flex items-center justify-end gap-3 border-t border-border px-5 py-4">
				<button
					onclick={onClose}
					class="rounded-md border border-border px-4 py-2 text-sm font-medium text-text transition-colors hover:bg-surface-raised"
				>
					Cancel
				</button>
				<button
					onclick={selectCurrent}
					class="rounded-md bg-accent px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-accent-hover"
				>
					Select "{breadcrumbs[breadcrumbs.length - 1]?.name ?? ''}"
				</button>
			</div>
		</div>
	</div>
{/if}
