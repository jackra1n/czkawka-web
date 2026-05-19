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
	let tooltipEl = $state<HTMLElement>();
	let timeout: ReturnType<typeof setTimeout>;
	let rect = $state<DOMRect>();

	let measured = $state(false);
	let coords = $state({ top: 0, left: 0 });

	function enter() {
		if (triggerEl) rect = triggerEl.getBoundingClientRect();
		clearTimeout(timeout);
		timeout = setTimeout(() => (show = true), 500);
	}

	function leave() {
		clearTimeout(timeout);
		show = false;
		rect = undefined;
		measured = false;
	}

	$effect(() => {
		return () => clearTimeout(timeout);
	});

	$effect(() => {
		if (show && rect && tooltipEl) {
			const tooltipWidth = tooltipEl.offsetWidth;
			const tooltipHeight = tooltipEl.offsetHeight;
			const viewportWidth = window.innerWidth;
			const viewportHeight = window.innerHeight;

			let x = 0;
			let y = 0;

			if (position === 'top') {
				x = rect.left + rect.width / 2 - tooltipWidth / 2;
				y = rect.top - tooltipHeight - 8;
			} else if (position === 'bottom') {
				x = rect.left + rect.width / 2 - tooltipWidth / 2;
				y = rect.bottom + 8;
			} else if (position === 'left') {
				x = rect.left - tooltipWidth - 8;
				y = rect.top + rect.height / 2 - tooltipHeight / 2;
			} else if (position === 'right') {
				x = rect.right + 8;
				y = rect.top + rect.height / 2 - tooltipHeight / 2;
			}

			// Clamp to viewport boundaries
			const padding = 8;
			x = Math.max(padding, Math.min(viewportWidth - tooltipWidth - padding, x));
			y = Math.max(padding, Math.min(viewportHeight - tooltipHeight - padding, y));

			coords = { top: y, left: x };
			measured = true;
		}
	});

	let arrowStyle = $derived.by(() => {
		if (!rect || !measured) return '';
		if (position === 'top' || position === 'bottom') {
			const arrowLeft = rect.left + rect.width / 2 - coords.left;
			const minArrowLeft = 6;
			const maxArrowLeft = (tooltipEl?.offsetWidth ?? 0) - 6;
			const clampedArrowLeft = Math.max(minArrowLeft, Math.min(maxArrowLeft, arrowLeft));
			return `left: ${clampedArrowLeft}px;`;
		} else {
			const arrowTop = rect.top + rect.height / 2 - coords.top;
			const minArrowTop = 6;
			const maxArrowTop = (tooltipEl?.offsetHeight ?? 0) - 6;
			const clampedArrowTop = Math.max(minArrowTop, Math.min(maxArrowTop, arrowTop));
			return `top: ${clampedArrowTop}px;`;
		}
	});

	const arrowPosMap: Record<string, string> = {
		top: 'bottom-0 -translate-x-1/2 translate-y-full border-x-[5px] border-t-[5px] border-x-transparent border-t-text',
		bottom: 'top-0 -translate-x-1/2 -translate-y-full border-x-[5px] border-b-[5px] border-x-transparent border-b-text',
		left: 'right-0 -translate-y-1/2 translate-x-full border-y-[5px] border-l-[5px] border-y-transparent border-l-text',
		right: 'left-0 -translate-y-1/2 -translate-x-full border-y-[5px] border-r-[5px] border-y-transparent border-r-text'
	};
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
	<div
		bind:this={tooltipEl}
		class="pointer-events-none fixed z-[999] max-w-xs rounded-md bg-text px-2.5 py-1.5 text-xs text-[#444] shadow-lg"
		style="top: {coords.top}px; left: {coords.left}px; visibility: {measured ? 'visible' : 'hidden'};"
		transition:fade={{ duration: 150 }}
	>
		{content}
		<div class="absolute h-0 w-0 {arrowPosMap[position]}" style={arrowStyle}></div>
	</div>
{/if}
