<script lang="ts">
	import type { ExifTag } from '$lib/api';

	let { tags }: { tags: ExifTag[] } = $props();

	type TagGroup = { name: string; tags: ExifTag[] };

	let groups = $derived.by<TagGroup[]>(() => {
		const buckets: Record<string, ExifTag[]> = Object.create(null);
		// The same tag can appear in multiple IFDs (e.g. XResolution in the main and
		// thumbnail directory), so collapse exact duplicates to one chip per tag.
		const seen: Record<string, true> = Object.create(null);
		for (const tag of tags) {
			const dedupeKey = `${tag.group}-${tag.code}-${tag.name}`;
			if (seen[dedupeKey]) continue;
			seen[dedupeKey] = true;
			const key = tag.group.trim() || 'Other';
			(buckets[key] ??= []).push(tag);
		}
		return Object.entries(buckets)
			.map(([name, groupTags]) => ({
				name,
				tags: [...groupTags].sort((a, b) => a.name.localeCompare(b.name)),
			}))
			.sort((a, b) => a.name.localeCompare(b.name));
	});
</script>

{#if tags.length === 0}
	<p class="text-sm text-text-muted">No EXIF tags.</p>
{:else}
	<div class="space-y-4">
		{#each groups as group (group.name)}
			<div>
				<p class="mb-1.5 text-xs font-medium tracking-wider text-text-muted uppercase">
					{group.name}
					<span class="text-text-muted/60">({group.tags.length})</span>
				</p>
				<div class="flex flex-wrap gap-1.5">
					{#each group.tags as tag, i (`${tag.code}-${tag.name}-${i}`)}
						<span
							class="rounded-md border border-border bg-surface-raised px-2 py-0.5 text-xs text-text"
							title={`Code 0x${tag.code.toString(16).padStart(4, '0')} (${tag.code})`}
						>
							{tag.name}
						</span>
					{/each}
				</div>
			</div>
		{/each}
	</div>
{/if}
