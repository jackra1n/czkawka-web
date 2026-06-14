<script lang="ts">
	import type { ExifTag } from '$lib/api';
	import Tooltip from './ui/Tooltip.svelte';

	let { tags, fallback }: { tags: ExifTag[] | undefined; fallback?: string } = $props();

	// Hovering a file with dozens of tags shouldn't produce a wall of text — cap the
	// tooltip and point at the preview panel, which shows the full grouped breakdown.
	const TOOLTIP_LIMIT = 12;

	let count = $derived(tags?.length ?? 0);
	let names = $derived((tags ?? []).map((t) => t.name));
	let inline = $derived(names.join(', '));
	let label = $derived(`${count} tag${count === 1 ? '' : 's'}`);
	let tooltip = $derived(
		names.length > TOOLTIP_LIMIT
			? `${names.slice(0, TOOLTIP_LIMIT).join(', ')} +${names.length - TOOLTIP_LIMIT} more — select to view all`
			: inline,
	);
</script>

{#if tags && tags.length > 0}
	<Tooltip class="flex min-w-0" content={tooltip}>
		<div class="flex min-w-0 items-center gap-1.5 text-text">
			<span class="shrink-0 text-text-muted">{label}</span>
			<span class="truncate text-xs text-text-muted">· {inline}</span>
		</div>
	</Tooltip>
{:else}
	<div class="truncate text-text">{fallback ?? label}</div>
{/if}
