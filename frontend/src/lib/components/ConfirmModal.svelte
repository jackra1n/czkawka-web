<script lang="ts">
	interface Props {
		open: boolean;
		title?: string;
		message?: string;
		confirmText?: string;
		cancelText?: string;
		onConfirm: () => void;
		onCancel: () => void;
	}

	let {
		open = false,
		title = 'Confirm',
		message = 'Are you sure?',
		confirmText = 'Confirm',
		cancelText = 'Cancel',
		onConfirm,
		onCancel
	}: Props = $props();
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
		onclick={(e) => { if (e.target === e.currentTarget) onCancel(); }}
		role="presentation"
	>
		<div
			class="w-full max-w-sm rounded-lg border border-border bg-surface p-6 shadow-xl"
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<h3 class="mb-2 text-lg font-semibold text-text">{title}</h3>
			<p class="mb-6 text-sm text-text-muted leading-relaxed">{message}</p>
			<div class="flex justify-end gap-3">
				<button
					type="button"
					onclick={onCancel}
					class="rounded-md border border-border bg-surface px-4 py-2 text-sm font-medium text-text transition-colors hover:bg-surface-raised"
				>
					{cancelText}
				</button>
				<button
					type="button"
					onclick={onConfirm}
					class="rounded-md bg-danger px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-danger-hover"
				>
					{confirmText}
				</button>
			</div>
		</div>
	</div>
{/if}
