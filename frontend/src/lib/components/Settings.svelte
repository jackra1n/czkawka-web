<script lang="ts">
	import Checkbox from './ui/Checkbox.svelte';

	let {
		excludedItems = $bindable<string>(),
		hideHardLinks = $bindable<boolean>(),
		onClose,
	}: {
		excludedItems: string;
		hideHardLinks: boolean;
		onClose: () => void;
	} = $props();
</script>

<div class="flex h-full flex-col overflow-y-auto bg-bg p-6">
	<div class="mx-auto flex w-full max-w-4xl flex-col gap-6">
		<div class="flex items-center justify-between">
			<h2 class="text-lg font-semibold text-text">Settings</h2>
			<button
				type="button"
				onclick={onClose}
				class="rounded-md px-3 py-1.5 text-sm text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
			>
				Back to tools
			</button>
		</div>

		<section class="flex flex-col gap-3 rounded-lg border border-border p-4">
			<h3 class="text-sm font-semibold text-text">Scan defaults</h3>

			<div class="flex flex-col gap-1.5">
				<label for="settings-excluded-items" class="text-xs font-medium text-text-muted">Excluded items</label>
				<input
					id="settings-excluded-items"
					type="text"
					bind:value={excludedItems}
					placeholder="*/.git/*,*/node_modules/*"
					class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:ring-1 focus:ring-accent focus:outline-none"
				/>
				<span class="text-xs text-text-muted">Comma-separated wildcard patterns (e.g. <code>*/.git/*</code>).</span>
			</div>

			<div class="flex flex-col gap-1">
				<Checkbox checked={hideHardLinks} onchange={(v) => (hideHardLinks = v)}>
					<span class="text-sm">Hide hard links</span>
				</Checkbox>
				<span class="text-xs text-text-muted">When two files are hard links of the same data, only one is reported as a duplicate result.</span>
			</div>
		</section>
	</div>
</div>
