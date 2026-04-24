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
		Lock
	} from 'lucide-svelte';

	const tools = [
		{ id: 'duplicates', label: 'Duplicate Files', icon: Copy, disabled: false },
		{ id: 'empty-folders', label: 'Empty Folders', icon: FolderOpen, disabled: false },
		{ id: 'big-files', label: 'Big Files', icon: BarChart3, disabled: false },
		{ id: 'empty-files', label: 'Empty Files', icon: FileX, disabled: false },
		{ id: 'temporary', label: 'Temporary Files', icon: Clock, disabled: false },
		{ id: 'similar-images', label: 'Similar Images', icon: Images, disabled: false },
		{ id: 'similar-videos', label: 'Similar Videos', icon: Video, disabled: false },
		{ id: 'same-music', label: 'Same Music', icon: Music, disabled: false },
		{ id: 'invalid-symlinks', label: 'Invalid Symlinks', icon: Link, disabled: false },
		{ id: 'broken-files', label: 'Broken Files', icon: AlertTriangle, disabled: false },
		{ id: 'bad-extensions', label: 'Bad Extensions', icon: FileCode, disabled: false },
		{ id: 'exif-remover', label: 'Exif Remover', icon: Camera, disabled: false },
		{ id: 'bad-names', label: 'Bad Names', icon: Type, disabled: false }
	];

	let { activeTool, onChangeTool }: { activeTool: string; onChangeTool: (toolId: string) => void } = $props();
</script>

<aside class="flex w-56 shrink-0 flex-col overflow-y-auto border-r border-border bg-surface">
	<div class="flex flex-col gap-0.5 p-2">
		{#each tools as tool (tool.id)}
			<button
				class="flex items-center gap-2.5 rounded-md px-3 py-2 text-left text-sm transition-colors"
				class:bg-surface-raised={tool.id === activeTool}
				class:text-text={tool.id === activeTool}
				class:text-text-muted={tool.id !== activeTool}
				class:hover:bg-surface-raised={!tool.disabled}
				class:hover:text-text={!tool.disabled}
				class:opacity-40={tool.disabled}
				class:cursor-not-allowed={tool.disabled}
				disabled={tool.disabled}
				onclick={() => {
					if (!tool.disabled && tool.id !== activeTool) onChangeTool(tool.id);
				}}
			>
				{#if tool.disabled}
					<Lock class="h-3.5 w-3.5 shrink-0 opacity-60" />
				{:else}
					<tool.icon class="h-4 w-4 shrink-0" />
				{/if}
				<span class="truncate">{tool.label}</span>
			</button>
		{/each}
	</div>
</aside>
