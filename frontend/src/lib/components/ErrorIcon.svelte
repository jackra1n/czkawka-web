<script lang="ts">
	import { File } from 'lucide-svelte';

	let {
		label,
		isDanger = false
	}: {
		label: string;
		isDanger?: boolean;
	} = $props();
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
	<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
		<File class="h-12 w-12 text-text-muted opacity-40" />
	</div>
	<p class={isDanger ? 'text-xs text-danger' : 'text-xs text-text-muted'}>{label}</p>
</div>