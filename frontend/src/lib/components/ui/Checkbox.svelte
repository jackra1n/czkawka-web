<script lang="ts">
	import { Check } from 'lucide-svelte';

	let {
		checked = false,
		onchange,
		disabled = false,
		children
	}: {
		checked?: boolean;
		onchange: (checked: boolean) => void;
		disabled?: boolean;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<label class="inline-flex cursor-pointer items-center gap-1.5 text-sm text-text {disabled ? 'cursor-not-allowed opacity-50' : ''}">
	<input
		type="checkbox"
		class="peer sr-only"
		{checked}
		{disabled}
		onchange={(e) => onchange(e.currentTarget.checked)}
	/>
	<div class="flex h-4 w-4 shrink-0 items-center justify-center rounded border border-border bg-bg transition-colors peer-checked:hidden peer-focus-visible:ring-1 peer-focus-visible:ring-accent"></div>
	<div class="hidden h-4 w-4 shrink-0 items-center justify-center rounded border border-accent bg-accent transition-colors peer-checked:flex peer-focus-visible:ring-1 peer-focus-visible:ring-accent">
		<Check class="h-3 w-3 text-bg" />
	</div>
	{#if children}
		<span>{@render children()}</span>
	{/if}
</label>
