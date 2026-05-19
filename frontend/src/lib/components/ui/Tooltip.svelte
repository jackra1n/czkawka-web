<script lang="ts">
	import { fade } from 'svelte/transition';

	let {
		content,
		position = 'top',
		class: className = 'inline-flex',
		children
	}: {
		content: string;
		position?: 'top' | 'bottom' | 'left' | 'right';
		class?: string;
		children: import('svelte').Snippet;
	} = $props();

	let show = $state(false);
	let triggerEl = $state<HTMLElement>();
	let timeout: ReturnType<typeof setTimeout>;
	let rect = $state<DOMRect>();

	function enter() {
		if (triggerEl) rect = triggerEl.getBoundingClientRect();
		clearTimeout(timeout);
		timeout = setTimeout(() => (show = true), 500);
	}

	function leave() {
		clearTimeout(timeout);
		show = false;
		rect = undefined;
	}

	$effect(() => {
		return () => clearTimeout(timeout);
	});
</script>

<div
	bind:this={triggerEl}
	onmouseenter={enter}
	onmouseleave={leave}
	onfocusin={enter}
	onfocusout={leave}
	class={className}
	role="none"
>
	{@render children()}
</div>

{#if show && rect}
	{@const topMap = {
		top: rect.top,
		bottom: rect.bottom,
		left: rect.top + rect.height / 2,
		right: rect.top + rect.height / 2
	}}
	{@const leftMap = {
		top: rect.left + rect.width / 2,
		bottom: rect.left + rect.width / 2,
		left: rect.left,
		right: rect.right
	}}
	{@const translateMap = {
		top: 'translate(-50%, -100%)',
		bottom: 'translate(-50%, 0)',
		left: 'translate(-100%, -50%)',
		right: 'translate(0, -50%)'
	}}
	{@const marginMap = {
		top: `margin-top: -8px`,
		bottom: `margin-top: 8px`,
		left: `margin-left: -8px`,
		right: `margin-left: 8px`
	}}

	{@const arrowPosMap: Record<string, string> = {
		top: 'bottom-0 left-1/2 -translate-x-1/2 translate-y-full border-x-[5px] border-t-[5px] border-x-transparent border-t-text',
		bottom: 'top-0 left-1/2 -translate-x-1/2 -translate-y-full border-x-[5px] border-b-[5px] border-x-transparent border-b-text',
		left: 'right-0 top-1/2 translate-x-full -translate-y-1/2 border-y-[5px] border-l-[5px] border-y-transparent border-l-text',
		right: 'left-0 top-1/2 -translate-x-full -translate-y-1/2 border-y-[5px] border-r-[5px] border-y-transparent border-r-text'
	}}

	<div
		class="pointer-events-none fixed z-[999] max-w-xs rounded-md bg-text px-2.5 py-1.5 text-xs text-[#444] shadow-lg"
		style="top: {topMap[position]}px; left: {leftMap[position]}px; transform: {translateMap[position]}; {marginMap[position]}"
		transition:fade={{ duration: 150 }}
	>
		{content}
		<div class="absolute h-0 w-0 {arrowPosMap[position]}"></div>
	</div>
{/if}
