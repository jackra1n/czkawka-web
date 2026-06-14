<script lang="ts">
	import type { ExifTag } from '$lib/api';
	import Tooltip from './ui/Tooltip.svelte';

	let { tags, fallback }: { tags: ExifTag[] | undefined; fallback?: string } = $props();

	let count = $derived(tags?.length ?? 0);
	let names = $derived((tags ?? []).map((t) => t.name).join(', '));
	let label = $derived(`${count} tag${count === 1 ? '' : 's'}`);
</script>

{#if tags && tags.length > 0}
	<Tooltip class="flex min-w-0" content={names}>
		<div class="flex min-w-0 items-center gap-1.5 text-text">
			<span class="shrink-0 text-text-muted">{label}</span>
			<span class="truncate text-xs text-text-muted">· {names}</span>
		</div>
	</Tooltip>
{:else}
	<div class="truncate text-text">{fallback ?? label}</div>
{/if}
