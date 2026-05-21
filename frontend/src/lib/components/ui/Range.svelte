<script lang="ts">
	let {
		id,
		min = 0,
		max = 100,
		step = 1,
		value = 0,
		onchange,
		disabled = false,
	}: {
		id?: string;
		min?: number;
		max?: number;
		step?: number;
		value?: number;
		onchange: (value: number) => void;
		disabled?: boolean;
	} = $props();

	const fillPct = $derived(((value - min) / (max - min)) * 100);
</script>

<input
	{id}
	type="range"
	{min}
	{max}
	{step}
	{value}
	oninput={(e) => onchange(Number(e.currentTarget.value))}
	{disabled}
	style="background: linear-gradient(to right, var(--color-accent) {fillPct}%, var(--color-border) {fillPct}%)"
	class="h-1.5 flex-1 cursor-pointer appearance-none rounded-full border-0 disabled:cursor-not-allowed disabled:opacity-50 [&::-moz-range-progress]:h-1.5 [&::-moz-range-progress]:rounded-full [&::-moz-range-progress]:bg-accent [&::-moz-range-thumb]:h-3.5 [&::-moz-range-thumb]:w-3.5 [&::-moz-range-thumb]:rounded-full [&::-moz-range-thumb]:border-0 [&::-moz-range-thumb]:bg-accent [&::-webkit-slider-thumb]:h-3.5 [&::-webkit-slider-thumb]:w-3.5 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-accent [&::-webkit-slider-thumb]:shadow-sm [&::-webkit-slider-thumb]:transition-colors [&::-webkit-slider-thumb]:hover:bg-accent-hover"
/>
