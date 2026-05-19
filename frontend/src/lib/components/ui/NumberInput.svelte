<script lang="ts">
	let {
		id,
		min = 0,
		max = 999999,
		value = 0,
		onchange,
		disabled = false
	}: {
		id?: string;
		min?: number;
		max?: number;
		value?: number;
		onchange: (value: number) => void;
		disabled?: boolean;
	} = $props();

	function decrement() {
		const next = Math.max(min, value - 1);
		if (next !== value) onchange(next);
	}

	function increment() {
		const next = Math.min(max, value + 1);
		if (next !== value) onchange(next);
	}

	function handleInput(e: Event) {
		const raw = (e.target as HTMLInputElement).value;
		const num = raw === '' ? min : Number(raw);
		onchange(Math.max(min, Math.min(max, num)));
	}

	const atMin = $derived(value <= min);
	const atMax = $derived(value >= max);
</script>

<div class="flex">
	<button
		type="button"
		onclick={decrement}
		disabled={disabled || atMin}
		class="flex w-9 shrink-0 items-center justify-center rounded-l-md border border-r-0 border-border bg-surface text-text-muted text-sm transition-colors hover:bg-surface-raised hover:text-text disabled:cursor-not-allowed disabled:opacity-50"
		aria-label="Decrease"
	>
		&minus;
	</button>
	<input
		{id}
		type="number"
		{min}
		{max}
		value={value}
		oninput={handleInput}
		{disabled}
		class="w-full min-w-0 border-y border-border bg-bg px-2 py-2 text-center text-sm text-text focus:border-accent focus:ring-1 focus:ring-inset focus:ring-accent focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
	/>
	<button
		type="button"
		onclick={increment}
		disabled={disabled || atMax}
		class="flex w-9 shrink-0 items-center justify-center rounded-r-md border border-l-0 border-border bg-surface text-text-muted text-sm transition-colors hover:bg-surface-raised hover:text-text disabled:cursor-not-allowed disabled:opacity-50"
		aria-label="Increase"
	>
		+
	</button>
</div>
