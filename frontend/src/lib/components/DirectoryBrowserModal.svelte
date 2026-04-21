<script lang="ts">
	import { X, Folder, ChevronRight } from 'lucide-svelte';

	interface Props {
		open: boolean;
		onClose: () => void;
		onSelect: (path: string) => void;
	}

	let { open, onClose, onSelect }: Props = $props();

	interface MockNode {
		name: string;
		path: string;
		children?: MockNode[];
	}

	const mockTree: MockNode = {
		name: '/',
		path: '/',
		children: [
			{
				name: 'home',
				path: '/home',
				children: [
					{
						name: 'user',
						path: '/home/user',
						children: [
							{ name: 'Downloads', path: '/home/user/Downloads' },
							{ name: 'Desktop', path: '/home/user/Desktop' },
							{ name: 'Documents', path: '/home/user/Documents' },
							{ name: 'Pictures', path: '/home/user/Pictures' },
							{ name: 'Music', path: '/home/user/Music' },
							{ name: 'Videos', path: '/home/user/Videos' }
						]
					},
					{
						name: 'admin',
						path: '/home/admin',
						children: [
							{ name: 'Projects', path: '/home/admin/Projects' },
							{ name: 'Backups', path: '/home/admin/Backups' }
						]
					}
				]
			},
			{
				name: 'etc',
				path: '/etc',
				children: [
					{ name: 'nginx', path: '/etc/nginx' },
					{ name: 'systemd', path: '/etc/systemd' }
				]
			},
			{
				name: 'var',
				path: '/var',
				children: [
					{ name: 'log', path: '/var/log' },
					{ name: 'tmp', path: '/var/tmp' }
				]
			},
			{
				name: 'tmp',
				path: '/tmp',
				children: [
					{ name: 'build', path: '/tmp/build' },
					{ name: 'cache', path: '/tmp/cache' }
				]
			},
			{
				name: 'usr',
				path: '/usr',
				children: [
					{ name: 'local', path: '/usr/local' },
					{ name: 'share', path: '/usr/share' }
				]
			}
		]
	};

	let currentPath = $state('/');

	function getNodeAtPath(path: string): MockNode | null {
		if (path === '/') return mockTree;
		const parts = path.split('/').filter(Boolean);
		let node: MockNode | undefined = mockTree;
		for (const part of parts) {
			if (!node?.children) return null;
			node = node.children.find((c) => c.name === part);
			if (!node) return null;
		}
		return node;
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
	}

	function selectCurrent() {
		onSelect(currentPath);
		onClose();
	}

	let currentNode = $derived(getNodeAtPath(currentPath));
	let breadcrumbs = $derived(getBreadcrumbs(currentPath));
	let folders = $derived(currentNode?.children?.filter((c) => c.children) ?? []);

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') onClose();
	}
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
				{#each folders as folder}
					<button
						class="flex w-full items-center gap-3 rounded-md px-3 py-2.5 text-left text-sm text-text transition-colors hover:bg-surface-raised"
						onclick={() => navigateTo(folder.path)}
					>
						<Folder class="h-4 w-4 text-accent" />
						<span>{folder.name}</span>
					</button>
				{/each}
				{#if folders.length === 0 && currentPath === '/'}
					<p class="px-3 py-8 text-center text-sm text-text-muted">No folders available.</p>
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
