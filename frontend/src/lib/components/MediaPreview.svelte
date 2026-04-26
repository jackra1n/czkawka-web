<script lang="ts">
	import { File, Loader2 } from 'lucide-svelte';
	import ErrorIcon from './ErrorIcon.svelte';

	let {
		previewType,
		fileUrl,
		mediaError,
		textContent,
		textLoading,
		textError,
		onMediaError,
		onDefaultVolume
	}: {
		previewType: 'video' | 'audio' | 'text' | 'unknown';
		fileUrl: string;
		mediaError: boolean;
		textContent: string;
		textLoading: boolean;
		textError: string;
		onMediaError: () => void;
		onDefaultVolume: (e: Event) => void;
	} = $props();
</script>

{#if previewType === 'video'}
	{#if mediaError}
		<ErrorIcon label="Failed to load video" />
	{:else}
		<div class="flex flex-1 items-center justify-center bg-bg p-4">
			<!-- svelte-ignore a11y_media_has_caption -->
			<video
				src={fileUrl}
				controls
				class="max-h-full max-w-full rounded-md"
				onerror={onMediaError}
				onloadedmetadata={onDefaultVolume}
			></video>
		</div>
	{/if}
{:else if previewType === 'audio'}
	{#if mediaError}
		<ErrorIcon label="Failed to load audio" />
	{:else}
		<div class="flex flex-1 flex-col items-center justify-center gap-4 bg-bg p-6">
			<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border">
				<File class="h-12 w-12 text-text-muted opacity-40" />
			</div>
			<audio
				src={fileUrl}
				controls
				class="w-full"
				onerror={onMediaError}
				onloadedmetadata={onDefaultVolume}
			></audio>
		</div>
	{/if}
{:else if previewType === 'text'}
	{#if textLoading}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 p-6">
			<Loader2 class="h-6 w-6 animate-spin text-text-muted" />
			<p class="text-xs text-text-muted">Loading text...</p>
		</div>
	{:else if textError}
		<ErrorIcon label={textError} isDanger={true} />
	{:else}
		<div class="flex-1 overflow-auto bg-bg p-4">
			<pre class="font-mono text-xs break-all whitespace-pre-wrap text-text">{textContent}</pre>
		</div>
	{/if}
{:else}
	<ErrorIcon label="No preview available" />
{/if}
