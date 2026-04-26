<script lang="ts">
	import { Columns2, MoveHorizontal, Layers, Image } from 'lucide-svelte';
	import { getFileUrl } from '$lib/api';
	import type { ScannedFile } from '$lib/api';

	type CompareMode = 'single' | 'side-by-side' | 'swipe' | 'onion';

	let {
		selectedFile,
		imageSiblings,
		compareMode,
		compareTarget,
		setCompareMode,
		setCompareTarget
	}: {
		selectedFile: string;
		imageSiblings: ScannedFile[];
		compareMode: CompareMode;
		compareTarget: string | null;
		setCompareMode: (mode: CompareMode) => void;
		setCompareTarget: (path: string) => void;
	} = $props();
</script>

<div class="shrink-0 space-y-2 border-b border-border px-4 py-2">
	<div class="flex items-center gap-1">
		<button
			onclick={() => setCompareMode('single')}
			class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
			'single'
				? 'bg-surface-raised text-text'
				: ''}"
			title="Single"
		>
			<Image class="h-3.5 w-3.5" />
		</button>
		<button
			onclick={() => setCompareMode('side-by-side')}
			class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
			'side-by-side'
				? 'bg-surface-raised text-text'
				: ''}"
			title="Side by side"
		>
			<Columns2 class="h-3.5 w-3.5" />
		</button>
		<button
			onclick={() => setCompareMode('swipe')}
			class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
			'swipe'
				? 'bg-surface-raised text-text'
				: ''}"
			title="Swipe"
		>
			<MoveHorizontal class="h-3.5 w-3.5" />
		</button>
		<button
			onclick={() => setCompareMode('onion')}
			class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
			'onion'
				? 'bg-surface-raised text-text'
				: ''}"
			title="Onion skin"
		>
			<Layers class="h-3.5 w-3.5" />
		</button>
	</div>
	{#if compareMode !== 'single'}
		<div class="flex items-center gap-2 overflow-x-auto">
			{#each imageSiblings as sibling (sibling.path)}
				<button
					onclick={() => {
						if (sibling.path !== selectedFile) setCompareTarget(sibling.path);
					}}
					class="relative shrink-0 overflow-hidden rounded border {sibling.path === selectedFile
						? 'border-primary'
						: sibling.path === compareTarget
							? 'border-primary ring-primary ring-1'
							: 'border-border'}"
					title={sibling.path}
				>
					<img
						src={getFileUrl(sibling.path)}
						alt=""
						class="h-8 w-8 object-cover"
						draggable="false"
					/>
				</button>
			{/each}
		</div>
	{/if}
</div>
