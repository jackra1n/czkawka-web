<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import { api, type ScanResults as ScanResultsType, type AppState, type ToolConfig } from '$lib/api';
	import DirectoryBrowserModal from '$lib/components/DirectoryBrowserModal.svelte';
	import ToolSidebar from '$lib/components/ToolSidebar.svelte';
	import ScanConfig from '$lib/components/ScanConfig.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	import FilePreview from '$lib/components/FilePreview.svelte';
	import { loadUiState, saveUiState, type UiState } from '$lib/stores/uiState';
	import { onDestroy } from 'svelte';

	let backendState = $state<AppState | null>(null);
	let stateLoaded = $state(false);

	let includedDirs = $state<string[]>([]);
	let excludedDirs = $state<string[]>([]);
	let excludedItems = $state<string>('');

	let uiState = $state<UiState>(loadUiState());
	let activeTool = $state(uiState.activeTool);
	let selectedFile = $state<string | null>(uiState.selectedFile);

	let scanState = $state<'idle' | 'running' | 'completed' | 'error'>('idle');
	let scanError = $state('');
	let scanResults = $state<ScanResultsType | null>(null);
	let scanId = $state('');
	let checkedFiles = $state<SvelteSet<string>>(new SvelteSet());
	let selectedFileSize = $state(0);

	let toolConfigs = $state<Record<string, ToolConfig>>({
		duplicates: {},
		'empty-files': {},
		'empty-folders': {},
		'big-files': {
			number_of_files: 50,
			search_mode: 'biggest'
		},
		'similar-images': {
			hash_alg: 'Gradient',
			hash_size: 16,
			resize_filter: 'Lanczos3',
			similarity: 5
		},
		'similar-videos': {
			tolerance: 5,
			vid_hash_duration: 10,
			crop_detect: 'Letterbox'
		},
		temporary: {}
	});

	let toolSelections = $state<Record<string, { path: string | null; size: number }>>({});

	let modalOpen = $state(false);
	let modalTarget: 'include' | 'exclude' = $state('include');

	let intervalId: ReturnType<typeof setInterval>;
	let dirsTimeout: ReturnType<typeof setTimeout>;
	let checkedTimeout: ReturnType<typeof setTimeout>;

	$effect(() => {
		loadState();
	});

	async function loadState() {
		try {
			const state = await api.getState();
			backendState = state;
			includedDirs = state.directories.included;
			excludedDirs = state.directories.excluded;
			excludedItems = state.directories.excluded_items;

			// Populate defaults on fresh state
			if (excludedDirs.length === 0 && excludedItems === '') {
				try {
					const defaults = await api.getDefaults();
					excludedDirs = defaults.excluded_directories;
					excludedItems = defaults.excluded_items;
				} catch (e) {
					console.error('Failed to load defaults:', e);
				}
			}

			restoreToolState(activeTool);
		} catch (err) {
			console.error('Failed to load state:', err);
		} finally {
			stateLoaded = true;
		}
	}

	function restoreToolState(toolId: string) {
		if (!backendState) return;

		const tool = backendState.tools[toolId];
		if (!tool) {
			scanState = 'idle';
			scanResults = null;
			scanError = '';
			scanId = '';
			checkedFiles = new SvelteSet();
			selectedFileSize = 0;
			return;
		}

		scanState = tool.status as typeof scanState;
		scanResults = tool.results ?? null;
		scanError = tool.error ?? '';
		scanId = tool.scan_id ?? '';
		checkedFiles = new SvelteSet(tool.checked_files ?? []);

		if (selectedFile && scanResults) {
			let foundSize = 0;
			for (const group of scanResults.groups) {
				for (const file of group.files) {
					if (file.path === selectedFile) {
						foundSize = group.size;
						break;
					}
				}
				if (foundSize) break;
			}
			selectedFileSize = foundSize;
		} else {
			selectedFileSize = 0;
		}

		if (scanState === 'running' && scanId) {
			poll();
			intervalId = setInterval(poll, 1000);
		}
	}

	function switchTool(toolId: string) {
		toolSelections[activeTool] = { path: selectedFile, size: selectedFileSize };
		activeTool = toolId;
		const saved = toolSelections[toolId];
		selectedFile = saved?.path ?? null;
		selectedFileSize = saved?.size ?? 0;
		restoreToolState(toolId);
	}

	// Persist UI state to localStorage
	$effect(() => {
		saveUiState({ activeTool, selectedFile });
	});

	// Auto-save directories to backend (debounced)
	$effect(() => {
		if (!stateLoaded) return;
		const included = [...includedDirs];
		const excluded = [...excludedDirs];
		const items = excludedItems;
		clearTimeout(dirsTimeout);
		dirsTimeout = setTimeout(() => {
			api.updateDirectories(included, excluded, items).catch(console.error);
		}, 500);
		return () => clearTimeout(dirsTimeout);
	});

	// Auto-save checked files to backend (debounced)
	$effect(() => {
		if (!stateLoaded) return;
		const files = Array.from(checkedFiles);
		clearTimeout(checkedTimeout);
		checkedTimeout = setTimeout(() => {
			api.updateToolState(activeTool, files).catch(console.error);
		}, 500);
		return () => clearTimeout(checkedTimeout);
	});

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
		if (activeTool === 'empty-folders') {
			selectedFile = null;
			selectedFileSize = 0;
			return;
		}
		selectedFile = file;
		selectedFileSize = size;
		toolSelections[activeTool] = { path: file, size };
	}

	function closePreview() {
		selectedFile = null;
		selectedFileSize = 0;
		toolSelections[activeTool] = { path: null, size: 0 };
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

	async function handleDelete() {
		const files = Array.from(checkedFiles);
		if (files.length === 0) return;

		try {
			const res = await api.deleteFiles(activeTool, files);
			scanError = '';

			for (const path of res.deleted) {
				checkedFiles.delete(path);
			}

			selectedFile = null;
			selectedFileSize = 0;
			toolSelections[activeTool] = { path: null, size: 0 };

			if (scanResults) {
				for (const group of scanResults.groups) {
					group.files = group.files.filter((f) => !res.deleted.includes(f.path));
				}
				const minSize = activeTool === 'empty-folders' || activeTool === 'big-files' || activeTool === 'empty-files' || activeTool === 'temporary' ? 1 : 2;
				scanResults.groups = scanResults.groups.filter((g) => g.files.length >= minSize);
				scanResults.total_groups = scanResults.groups.length;
				scanResults.total_items = scanResults.groups.reduce((sum, g) => sum + g.files.length, 0);
				scanResults.wasted_bytes = scanResults.groups.reduce(
					(sum, g) => sum + g.size * (g.files.length - 1),
					0
				);
			}

			if (res.failed.length > 0) {
				scanError = `Failed to delete ${res.failed.length} file${res.failed.length === 1 ? '' : 's'}`;
			}
		} catch (err) {
			scanError = err instanceof Error ? err.message : 'Failed to delete files';
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
		selectedFileSize = 0;

		const payload: Parameters<typeof api.startScan>[0] = {
			directories: dirs,
			exclude_directories: excluded.length > 0 ? excluded : undefined,
			excluded_items: excludedItems.trim() || undefined,
			min_file_size: 8192,
			tool_id: activeTool
		};

		if (activeTool === 'big-files') {
			const cfg = toolConfigs['big-files'];
			payload.number_of_files = cfg?.number_of_files;
			payload.search_mode = cfg?.search_mode;
		}

		if (activeTool === 'similar-videos') {
			const cfg = toolConfigs['similar-videos'];
			payload.tolerance = cfg?.tolerance;
			payload.vid_hash_duration = cfg?.vid_hash_duration;
			payload.crop_detect = cfg?.crop_detect;
		}

		if (activeTool === 'similar-images') {
			const cfg = toolConfigs['similar-images'];
			payload.hash_alg = cfg?.hash_alg;
			payload.hash_size = cfg?.hash_size;
			payload.resize_filter = cfg?.resize_filter;
			payload.similarity = cfg?.similarity;
		}

		try {
			const res = await api.startScan(payload);
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
		clearTimeout(dirsTimeout);
		clearTimeout(checkedTimeout);
	});
</script>

<div class="flex h-full w-full">
	<ToolSidebar {activeTool} onChangeTool={switchTool} />

	<div class="flex flex-1 flex-col min-h-0 overflow-hidden bg-bg">
		<ScanConfig
			bind:includedDirs
			bind:excludedDirs
			bind:excludedItems
			bind:activeTool
			bind:toolConfig={toolConfigs[activeTool]}
			{scanState}
			{scanResults}
			checkedFiles={checkedFiles}
			onStartScan={startScan}
			onAddDir={openModal}
			onDelete={handleDelete}
		/>

		<div class="flex flex-1 min-h-0 overflow-hidden">
			<ScanResults
				{scanState}
				{scanError}
				{scanResults}
				onSelectFile={selectFile}
				checkedFiles={checkedFiles}
				{activeTool}
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
