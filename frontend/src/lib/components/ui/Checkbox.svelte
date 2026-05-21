<script lang="ts">
	import { Check } from 'lucide-svelte';

	let {
		checked = false,
		onchange,
		disabled = false,
		reverse = false,
		class: className = '',
		children,
	}: {
		checked?: boolean;
		onchange: (checked: boolean) => void;
		disabled?: boolean;
		reverse?: boolean;
		class?: string;
		children?: import('svelte').Snippet;
	} = $props();
</script>

<label
	class="group inline-flex cursor-pointer items-center gap-1.5 text-sm text-text {disabled
		? 'cursor-not-allowed opacity-50'
		: ''} {className}"
>
	<input
		type="checkbox"
		class="peer sr-only"
		{checked}
		{disabled}
		onchange={(e) => onchange(e.currentTarget.checked)}
	/>
	{#if reverse && children}
		<span class="flex-1">{@render children()}</span>
	{/if}
	<div
		class="flex h-4 w-4 shrink-0 items-center justify-center rounded border border-text-muted/35 bg-surface-raised/40 transition-colors group-hover:border-text-muted/65 group-hover:bg-surface-raised/70 peer-checked:hidden peer-focus-visible:ring-1 peer-focus-visible:ring-accent"
	></div>
	<div
		class="hidden h-4 w-4 shrink-0 items-center justify-center rounded border border-accent bg-accent transition-colors peer-checked:flex peer-focus-visible:ring-1 peer-focus-visible:ring-accent"
	>
		<Check class="h-3 w-3 text-bg" />
	</div>
	{#if !reverse && children}
		<span>{@render children()}</span>
	{/if}
</label>
