<script lang="ts">
	import { Columns2, MoveHorizontal, Layers, Image, Settings, Check } from 'lucide-svelte';
	import { getFileUrl } from '$lib/api';
	import type { ScannedFile } from '$lib/api';
	import Tooltip from '../ui/Tooltip.svelte';

	type CompareMode = 'single' | 'side-by-side' | 'swipe' | 'onion';

	let {
		selectedFile,
		imageSiblings,
		compareMode,
		compareTarget,
		colorCodingEnabled,
		setCompareMode,
		setCompareTarget,
		toggleColorCoding
	}: {
		selectedFile: string;
		imageSiblings: ScannedFile[];
		compareMode: CompareMode;
		compareTarget: string | null;
		colorCodingEnabled: boolean;
		setCompareMode: (mode: CompareMode) => void;
		setCompareTarget: (path: string) => void;
		toggleColorCoding: () => void;
	} = $props();

	let settingsOpen = $state(false);
</script>

<div class="shrink-0 space-y-2 border-b border-border px-4 py-2">
	<div class="flex items-center justify-between">
		<div class="flex items-center gap-1">
			<Tooltip content="Single">
				<button
					onclick={() => setCompareMode('single')}
					class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
					'single'
						? 'bg-surface-raised text-text'
						: ''}"
				>
					<Image class="h-3.5 w-3.5" />
				</button>
			</Tooltip>
			<Tooltip content="Side by side">
				<button
					onclick={() => setCompareMode('side-by-side')}
					class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
					'side-by-side'
						? 'bg-surface-raised text-text'
						: ''}"
				>
					<Columns2 class="h-3.5 w-3.5" />
				</button>
			</Tooltip>
			<Tooltip content="Swipe">
				<button
					onclick={() => setCompareMode('swipe')}
					class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
					'swipe'
						? 'bg-surface-raised text-text'
						: ''}"
				>
					<MoveHorizontal class="h-3.5 w-3.5" />
				</button>
			</Tooltip>
			<Tooltip content="Onion skin">
				<button
					onclick={() => setCompareMode('onion')}
					class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {compareMode ===
					'onion'
						? 'bg-surface-raised text-text'
						: ''}"
				>
					<Layers class="h-3.5 w-3.5" />
				</button>
			</Tooltip>
		</div>
		<div class="relative">
			<Tooltip content="Settings">
				<button
					onclick={() => (settingsOpen = !settingsOpen)}
					class="rounded p-1.5 text-text-muted transition-colors hover:bg-surface-raised hover:text-text {settingsOpen
						? 'bg-surface-raised text-text'
						: ''}"
				>
					<Settings class="h-3.5 w-3.5" />
				</button>
			</Tooltip>
			{#if settingsOpen}
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div
					class="absolute right-0 z-30 mt-1 w-40 rounded-md border border-border bg-surface shadow-lg"
					onmouseleave={() => (settingsOpen = false)}
				>
					<button
						onclick={() => {
							toggleColorCoding();
							settingsOpen = false;
						}}
						class="flex w-full items-center justify-between gap-2 px-3 py-2 text-left text-xs text-text transition-colors hover:bg-surface-raised"
					>
						Color code
						<span class="flex h-3.5 w-3.5 items-center justify-center rounded border border-text-muted">
							{#if colorCodingEnabled}
								<Check class="h-3 w-3" />
							{/if}
						</span>
					</button>
				</div>
			{/if}
		</div>
	</div>
	{#if compareMode !== 'single'}
		<div class="flex items-center gap-2 overflow-x-auto">
			{#each imageSiblings as sibling (sibling.path)}
				<Tooltip content={sibling.path}>
					<button
						onclick={() => {
							if (sibling.path !== selectedFile) setCompareTarget(sibling.path);
						}}
						class="relative shrink-0 overflow-hidden rounded border {sibling.path === selectedFile
							? colorCodingEnabled
								? 'border-blue-500 ring-1 ring-blue-500'
								: 'border-primary'
							: sibling.path === compareTarget
								? colorCodingEnabled
									? 'border-amber-500 ring-1 ring-amber-500'
									: 'border-primary ring-primary ring-1'
								: 'border-border'}"
					>
						<img
							src={getFileUrl(sibling.path)}
							alt=""
							class="h-8 w-8 object-cover"
							draggable="false"
						/>
					</button>
				</Tooltip>
			{/each}
		</div>
	{/if}
</div>
