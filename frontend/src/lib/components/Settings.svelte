<script lang="ts">
	import Checkbox from './ui/Checkbox.svelte';
	import { ArrowLeft, Settings2, FolderX, EyeOff, RotateCcw } from 'lucide-svelte';

	let {
		excludedItems = $bindable<string>(),
		hideHardLinks = $bindable<boolean>(),
		defaultExcludedItems = '',
		onClose,
	}: {
		excludedItems: string;
		hideHardLinks: boolean;
		defaultExcludedItems?: string;
		onClose: () => void;
	} = $props();

	function resetToDefaults() {
		excludedItems = defaultExcludedItems;
		hideHardLinks = true;
	}
</script>

<div class="flex h-full flex-col bg-bg">
	<!-- Header Bar -->
	<header class="flex h-14 shrink-0 items-center justify-between border-b border-border bg-surface px-6">
		<div class="flex items-center gap-2.5">
			<Settings2 class="h-4.5 w-4.5 text-accent" />
			<h2 class="text-sm font-semibold text-text">Global Settings</h2>
		</div>
		<div class="flex items-center gap-3">
			<button
				type="button"
				onclick={resetToDefaults}
				class="flex items-center gap-1.5 rounded-md border border-border bg-surface-raised/40 px-3 py-1.5 text-xs font-medium text-text-muted transition-all hover:bg-surface-raised hover:text-accent hover:border-accent/40"
			>
				<RotateCcw class="h-3.5 w-3.5" />
				Reset to defaults
			</button>
			<button
				type="button"
				onclick={onClose}
				class="group flex items-center gap-2 rounded-md border border-border bg-surface-raised/50 px-3 py-1.5 text-xs font-medium text-text-muted transition-all hover:bg-surface-raised hover:text-text hover:border-text-muted/30"
			>
				<ArrowLeft class="h-3.5 w-3.5 transition-transform group-hover:-translate-x-0.5" />
				Back to tools
			</button>
		</div>
	</header>

	<!-- Content Area -->
	<div class="flex-1 overflow-y-auto p-6">
		<div class="mx-auto flex w-full max-w-3xl flex-col gap-6">
			
			<!-- Card 1: Exclusions -->
			<section class="flex flex-col gap-5 rounded-xl border border-border bg-surface p-6 shadow-md transition-all hover:border-border/80">
				<div class="flex items-center gap-2 border-b border-border/60 pb-3">
					<FolderX class="h-4 w-4 text-text-muted" />
					<h3 class="text-xs font-semibold uppercase tracking-wider text-text">Scan Exclusions</h3>
				</div>

				<div class="flex flex-col gap-2">
					<label for="settings-excluded-items" class="text-xs font-medium text-text-muted">Excluded items</label>
					<input
						id="settings-excluded-items"
						type="text"
						bind:value={excludedItems}
						placeholder="*/.git/*,*/node_modules/*"
						class="w-full rounded-lg border border-border bg-surface-raised px-3.5 py-2.5 text-sm text-text placeholder:text-text-muted focus:border-accent focus:ring-1 focus:ring-accent focus:outline-none transition-all"
					/>
					<span class="text-xs text-text-muted leading-relaxed">
						Comma-separated wildcard patterns (e.g. <code>*/.git/*</code>, <code>*.tmp</code>). Files or folders matching these patterns will be skipped during scanning.
					</span>
				</div>
			</section>

			<!-- Card 2: Duplicate Resolution -->
			<section class="flex flex-col gap-5 rounded-xl border border-border bg-surface p-6 shadow-md transition-all hover:border-border/80">
				<div class="flex items-center gap-2 border-b border-border/60 pb-3">
					<EyeOff class="h-4 w-4 text-text-muted" />
					<h3 class="text-xs font-semibold uppercase tracking-wider text-text">Result Filters</h3>
				</div>

				<div class="flex items-start gap-3 rounded-lg border border-border/55 bg-surface-raised/40 p-4 transition-colors hover:bg-surface-raised/60">
					<div class="pt-0.5">
						<Checkbox checked={hideHardLinks} onchange={(v) => (hideHardLinks = v)} />
					</div>
					<div class="flex flex-col gap-1">
						<span class="text-sm font-medium text-text">Hide hard links</span>
						<span class="text-xs text-text-muted leading-relaxed">
							When multiple duplicate files are hard links pointing to the exact same underlying disk data, only one of them will be listed in the duplicate results. Enabling this keeps your list clean and prevents you from resolving the same files multiple times.
						</span>
					</div>
				</div>
			</section>
			
		</div>
	</div>
</div>
