<script lang="ts">
	import './layout.css';
	import { api } from '$lib/api';
	import { onMount } from 'svelte';
	import Tooltip from '$lib/components/ui/Tooltip.svelte';

	let { children } = $props();

	let healthy = $state(true);

	onMount(() => {
		const check = () => {
			api
				.health()
				.then(() => (healthy = true))
				.catch(() => (healthy = false));
		};
		check();
		const id = setInterval(check, 5000);
		return () => clearInterval(id);
	});
</script>

<svelte:head>
	<title>czkawka-web</title>
</svelte:head>

<div class="flex h-dvh flex-col">
	<header class="flex shrink-0 items-center justify-between border-b border-border px-4 py-3">
		<div class="flex items-center gap-2.5">
			<div class="h-2.5 w-2.5 rounded-full bg-accent"></div>
			<span class="text-sm font-semibold tracking-tight">czkawka-web</span>
		</div>
		<div class="flex items-center gap-3">
			<a
				href="https://github.com/jackra1n/czkawka-web"
				target="_blank"
				rel="noopener noreferrer"
				class="flex items-center gap-1.5 text-xs text-text-muted transition-colors hover:text-text"
			>
				<svg
					class="h-4 w-4"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
				>
					<path
						d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
					/>
					<path d="M9 18c-4.51 2-5-2-7-2" />
				</svg>
				<span>Source Code</span>
			</a>
			<Tooltip position="bottom" content={healthy ? 'Backend connected' : 'Backend disconnected'}>
				<div
					class="h-2 w-2 rounded-full transition-colors duration-300"
					class:bg-success={healthy}
					class:bg-danger={!healthy}
				></div>
			</Tooltip>
		</div>
	</header>

	<div class="relative min-h-0 flex-1 overflow-hidden">
		{@render children()}
	</div>
</div>
