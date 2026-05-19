<script lang="ts">
	import {
		Copy,
		FolderOpen,
		BarChart3,
		FileX,
		Clock,
		Images,
		Video,
		Music,
		Link,
		AlertTriangle,
		FileCode,
		Camera,
		Type,
		PanelLeft,
		PanelLeftClose
	} from 'lucide-svelte';

	const tools = [
		{ id: 'duplicates', label: 'Duplicate Files', icon: Copy },
		{ id: 'empty-folders', label: 'Empty Folders', icon: FolderOpen },
		{ id: 'big-files', label: 'Big Files', icon: BarChart3 },
		{ id: 'empty-files', label: 'Empty Files', icon: FileX },
		{ id: 'temporary', label: 'Temporary Files', icon: Clock },
		{ id: 'similar-images', label: 'Similar Images', icon: Images },
		{ id: 'similar-videos', label: 'Similar Videos', icon: Video },
		{ id: 'same-music', label: 'Music Duplicates', icon: Music },
		{ id: 'invalid-symlinks', label: 'Invalid Symlinks', icon: Link },
		{ id: 'broken-files', label: 'Broken Files', icon: AlertTriangle },
		{ id: 'bad-extensions', label: 'Bad Extensions', icon: FileCode },
		{ id: 'bad-names', label: 'Bad Names', icon: Type },
		{ id: 'exif-remover', label: 'Exif Remover', icon: Camera }
	];

	let {
		activeTool,
		collapsed,
		onChangeTool,
		onToggleCollapse
	}: {
		activeTool: string;
		collapsed: boolean;
		onChangeTool: (toolId: string) => void;
		onToggleCollapse: () => void;
	} = $props();
</script>

<aside class="flex shrink-0 flex-col overflow-y-auto border-r border-border bg-surface" class:w-max={!collapsed} class:w-14={collapsed}>
	<div class="flex flex-col gap-px p-2">
		{#each tools as tool (tool.id)}
			<button
				type="button"
				class="flex h-9 w-full items-center gap-2.5 rounded-md px-3 text-left text-sm transition-colors whitespace-nowrap hover:bg-surface-raised hover:text-text {tool.id === activeTool ? 'bg-accent/15 text-accent font-medium' : 'text-text-muted'}"
				onclick={() => {
					if (tool.id !== activeTool) onChangeTool(tool.id);
				}}
			>
				<tool.icon class="h-4 w-4 shrink-0" />
				{#if !collapsed}
					<span class="truncate leading-none">{tool.label}</span>
				{/if}
			</button>
		{/each}
	</div>
	<div class="mt-auto p-2">
		<button
			type="button"
			onclick={onToggleCollapse}
			title={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
			class="flex h-9 w-full items-center gap-2.5 rounded-md px-3 text-left text-sm whitespace-nowrap text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
		>
			{#if collapsed}
				<PanelLeft class="h-4 w-4 shrink-0" />
			{:else}
				<PanelLeftClose class="h-4 w-4 shrink-0" />
				<span class="truncate leading-none">Collapse</span>
			{/if}
		</button>
	</div>
</aside>
