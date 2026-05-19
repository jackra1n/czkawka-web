<script lang="ts">
	import { ChevronDown } from 'lucide-svelte';
	import { fade } from 'svelte/transition';

	let {
		id,
		options,
		value,
		onchange,
		disabled = false
	}: {
		id?: string;
		options: { value: string; label: string }[];
		value: string;
		onchange: (value: string) => void;
		disabled?: boolean;
	} = $props();

	let open = $state(false);
	let triggerEl = $state<HTMLElement>();
	let rect = $state<DOMRect>();
	let activeIdx = $state(-1);
	let listboxId = $state('');

	const selected = $derived(options.find((o) => String(o.value) === String(value)) ?? options[0]);

	$effect(() => {
		listboxId = id ? `${id}-listbox` : `select-${Math.random().toString(36).slice(2, 9)}`;
	});

	function toggle() {
		if (disabled) return;
		if (!open && triggerEl) rect = triggerEl.getBoundingClientRect();
		open = !open;
		if (open) activeIdx = options.findIndex((o) => String(o.value) === String(value));
	}

	function select(val: string) {
		onchange(val);
		open = false;
	}

	function handleListboxClick(e: MouseEvent) {
		const target = (e.target as HTMLElement).closest('[data-value]');
		if (target) select(target.getAttribute('data-value')!);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowDown') {
			e.preventDefault();
			if (!open) { toggle(); return; }
			activeIdx = Math.min(activeIdx + 1, options.length - 1);
		}
		if (e.key === 'ArrowUp') {
			e.preventDefault();
			if (!open) { toggle(); return; }
			activeIdx = Math.max(activeIdx - 1, 0);
		}
		if (e.key === 'Enter' && open && activeIdx >= 0) {
			e.preventDefault();
			select(options[activeIdx].value);
		}
		if (e.key === 'Escape') {
			open = false;
			triggerEl?.focus();
		}
		if (e.key === ' ' && !open) {
			e.preventDefault();
			toggle();
		}
	}

	function handleClickOutside(e: MouseEvent) {
		if (triggerEl && !triggerEl.contains(e.target as Node)) {
			open = false;
		}
	}

	function handleScroll() {
		open = false;
	}

	$effect(() => {
		if (open) {
			document.addEventListener('click', handleClickOutside, true);
			window.addEventListener('scroll', handleScroll, true);
			window.addEventListener('resize', handleScroll);
			return () => {
				document.removeEventListener('click', handleClickOutside, true);
				window.removeEventListener('scroll', handleScroll, true);
				window.removeEventListener('resize', handleScroll);
			};
		}
	});
</script>

<div class="relative">
	<button
		{id}
		type="button"
		role="combobox"
		aria-expanded={open}
		aria-controls={listboxId}
		{disabled}
		bind:this={triggerEl}
		onclick={toggle}
		onkeydown={handleKeydown}
		class="flex w-full items-center justify-between rounded-md border border-border bg-bg px-3 py-2 text-left text-sm text-text transition-colors focus:border-accent focus:ring-1 focus:ring-accent focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 {open ? 'border-accent ring-1 ring-accent' : ''}"
	>
		<span class={selected ? '' : 'text-text-muted'}>{selected?.label ?? ''}</span>
		<ChevronDown
			class="h-4 w-4 shrink-0 text-text-muted transition-transform duration-200 {open
				? 'rotate-180'
				: ''}"
		/>
	</button>

	{#if open && rect}
		<div
			id={listboxId}
			class="fixed z-50 overflow-hidden rounded-md border border-border bg-surface shadow-lg"
			style="top: {rect.bottom + 4}px; left: {rect.left}px; min-width: {rect.width}px;"
			role="listbox"
			tabindex="-1"
			transition:fade={{ duration: 150 }}
			onclick={handleListboxClick}
			onmouseleave={() => (activeIdx = -1)}
			onkeydown={handleKeydown}
		>
			{#each options as opt, i (opt.value)}
				<div
					role="option"
					aria-selected={String(opt.value) === String(value)}
					tabindex="-1"
					data-value={opt.value}
					onmouseenter={() => (activeIdx = i)}
					class="cursor-pointer select-none px-3 py-2 text-sm transition-colors {String(opt.value) === String(value)
						? 'bg-accent/15 text-accent'
						: 'text-text hover:bg-surface-raised'} {i === activeIdx && String(opt.value) !== String(value)
						? 'bg-surface-raised'
						: ''}"
				>
					{opt.label}
				</div>
			{/each}
		</div>
	{/if}
</div>
