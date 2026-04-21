<script lang="ts">
	import { api, type ScanResults as ScanResultsType } from '$lib/api';
	import DirectoryBrowserModal from '$lib/components/DirectoryBrowserModal.svelte';
	import ToolSidebar from '$lib/components/ToolSidebar.svelte';
	import ScanConfig from '$lib/components/ScanConfig.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	import FilePreview from '$lib/components/FilePreview.svelte';
	import { onDestroy } from 'svelte';

	let includedDirs = $state<string[]>([]);
	let excludedDirs = $state<string[]>([]);

	let scanState = $state<'idle' | 'running' | 'completed' | 'error'>('idle');
	let scanError = $state('');
	let scanResults = $state<ScanResultsType | null>(null);
	let scanId = $state('');

	let selectedFile = $state<string | null>(null);
	let selectedFileSize = $state(0);

	let modalOpen = $state(false);
	let modalTarget: 'include' | 'exclude' = $state('include');

	let activeTool = $state('duplicates');

	let intervalId: ReturnType<typeof setInterval>;

	function openModal(target: 'include' | 'exclude') {
		modalTarget = target;
		modalOpen = true;
	}

	function handleModalSelect(path: string) {
		if (modalTarget === 'include') {
			includedDirs = [...includedDirs, path];
		} else {
			excludedDirs = [...excludedDirs, path];
		}
	}

	function selectFile(file: string | null, size: number) {
		selectedFile = file;
		selectedFileSize = size;
	}

	function closePreview() {
		selectedFile = null;
	}

	async function poll() {
		if (!scanId) return;
		try {
			const res = await api.getScanStatus(scanId);
			scanError = '';

			if (res.status === 'completed') {
				scanState = 'completed';
				scanResults = res.results ?? null;
				clearInterval(intervalId);
			} else if (res.status === 'error') {
				scanState = 'error';
				scanError = res.error ?? 'Unknown error';
				clearInterval(intervalId);
			} else if (res.status === 'not_found') {
				scanState = 'error';
				scanError = 'Scan not found';
				clearInterval(intervalId);
			}
		} catch (err) {
			scanState = 'error';
			scanError = err instanceof Error ? err.message : 'Failed to fetch status';
			clearInterval(intervalId);
		}
	}

	async function startScan() {
		const dirs = includedDirs.map((s) => s.trim()).filter(Boolean);

		if (dirs.length === 0) {
			scanError = 'Please enter at least one directory.';
			return;
		}

		const excluded = excludedDirs.map((s) => s.trim()).filter(Boolean);

		scanState = 'running';
		scanError = '';
		scanResults = null;
		selectedFile = null;

		try {
			const res = await api.startScan({
				directories: dirs,
				exclude_directories: excluded.length > 0 ? excluded : undefined,
				min_file_size: 8192
			});
			scanId = res.id;
			poll();
			intervalId = setInterval(poll, 1000);
		} catch (err) {
			scanState = 'error';
			scanError = err instanceof Error ? err.message : 'Failed to start scan';
		}
	}

	onDestroy(() => {
		clearInterval(intervalId);
	});
</script>

<div class="flex h-full w-full">
	<ToolSidebar bind:activeTool />

	<div class="flex flex-1 flex-col min-h-0 overflow-hidden bg-bg">
		<ScanConfig
			bind:includedDirs
			bind:excludedDirs
			{scanState}
			onStartScan={startScan}
			onAddDir={openModal}
		/>

		<div class="flex flex-1 min-h-0 overflow-hidden">
			<ScanResults
				{scanState}
				{scanError}
				{scanResults}
				onSelectFile={selectFile}
			/>

			{#if selectedFile}
				<FilePreview
					{selectedFile}
					{selectedFileSize}
					onClose={closePreview}
				/>
			{/if}
		</div>
	</div>
</div>

<DirectoryBrowserModal
	open={modalOpen}
	onClose={() => (modalOpen = false)}
	onSelect={handleModalSelect}
/>
