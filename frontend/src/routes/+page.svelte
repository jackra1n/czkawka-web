<script lang="ts">
	import { SvelteSet } from 'svelte/reactivity';
	import {
		api,
		type ScanResults as ScanResultsType,
		type AppState,
		type ToolConfig,
		type ScanProgress,
	} from '$lib/api';
	import DirectoryBrowserModal from '$lib/components/DirectoryBrowserModal.svelte';
	import ToolSidebar from '$lib/components/ToolSidebar.svelte';
	import ScanConfig from '$lib/components/ScanConfig.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	import FilePreview from '$lib/components/FilePreview.svelte';
	import { loadUiState, saveUiState, type UiState } from '$lib/stores/uiState';
	import { onDestroy } from 'svelte';
	import { DEFAULT_TOOL_CONFIGS } from '$lib/defaults';

	// ---- State ----
	let backendState = $state<AppState | null>(null);
	let stateLoaded = $state(false);

	let includedDirs = $state<string[]>([]);
	let excludedDirs = $state<string[]>([]);
	let defaultExcludedDirs = $state<string[]>([]);
	let excludedItems = $state<string>('');

	let uiState = $state<UiState>(loadUiState());
	let activeTool = $state(uiState.activeTool);
	let selectedFile = $state<string | null>(uiState.selectedFile);
	let sidebarCollapsed = $state(uiState.sidebarCollapsed);

	let scanState = $state<'idle' | 'running' | 'completed' | 'error'>('idle');
	let scanError = $state('');
	let scanResults = $state<ScanResultsType | null>(null);
	let scanProgress = $state<ScanProgress | null>(null);
	let scanId = $state('');
	let checkedFiles = $state<SvelteSet<string>>(new SvelteSet());
	let selectedFileSize = $state(0);

	let toolConfigs = $state<Record<string, ToolConfig>>(structuredClone(DEFAULT_TOOL_CONFIGS));

	let toolSelections = $state<Record<string, { path: string | null; size: number }>>({});
	let modalOpen = $state(false);
	let modalTarget: 'include' | 'exclude' = $state('include');

	let selectedFileGroup = $derived(
		selectedFile && scanResults
			? (scanResults.groups.find((g) => g.files.some((f) => f.path === selectedFile)) ?? null)
			: null,
	);

	let intervalId: ReturnType<typeof setInterval>;
	let dirsTimeout: ReturnType<typeof setTimeout>;
	let checkedTimeout: ReturnType<typeof setTimeout>;

	const SINGLE_FILE_TOOLS = new Set([
		'empty-folders',
		'big-files',
		'empty-files',
		'temporary',
		'invalid-symlinks',
		'broken-files',
		'bad-extensions',
		'exif-remover',
		'bad-names',
	]);

	// ---- Effects ----
	$effect(() => {
		loadState();
	});
	$effect(() => {
		saveUiState({ activeTool, selectedFile, sidebarCollapsed });
	});
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
	$effect(() => {
		if (!stateLoaded) return;
		const toolId = activeTool;
		const files = Array.from(checkedFiles);
		clearTimeout(checkedTimeout);
		checkedTimeout = setTimeout(() => {
			api.updateToolState(toolId, files).catch(console.error);
		}, 500);
		return () => clearTimeout(checkedTimeout);
	});

	onDestroy(() => {
		clearInterval(intervalId);
		clearTimeout(dirsTimeout);
		clearTimeout(checkedTimeout);
	});

	// ---- Helpers ----
	function groupSizeFor(path: string): number {
		if (!scanResults) return 0;
		for (const group of scanResults.groups) {
			for (const file of group.files) {
				if (file.path === path) return group.size;
			}
		}
		return 0;
	}

	function removeFromResults(paths: string[]) {
		if (!scanResults) return;
		for (const group of scanResults.groups) {
			group.files = group.files.filter((f) => !paths.includes(f.path));
		}
		const minSize = SINGLE_FILE_TOOLS.has(activeTool) ? 1 : 2;
		scanResults.groups = scanResults.groups.filter((g) => g.files.length >= minSize);
		scanResults.total_groups = scanResults.groups.length;
		scanResults.total_items = scanResults.groups.reduce((sum, g) => sum + g.files.length, 0);
		scanResults.wasted_bytes = scanResults.groups.reduce((sum, g) => sum + g.size * (g.files.length - 1), 0);
	}

	function buildPayload(): Parameters<typeof api.startScan>[0] {
		const cfg = toolConfigs[activeTool];
		const base: Parameters<typeof api.startScan>[0] = {
			directories: includedDirs.map((s) => s.trim()).filter(Boolean),
			exclude_directories:
				excludedDirs.map((s) => s.trim()).filter(Boolean).length > 0
					? excludedDirs.map((s) => s.trim()).filter(Boolean)
					: undefined,
			excluded_items: excludedItems.trim() || undefined,
			min_file_size: 8192,
			tool_id: activeTool,
		};

		switch (activeTool) {
			case 'big-files':
				return { ...base, number_of_files: cfg?.number_of_files, search_mode: cfg?.search_mode };
			case 'similar-videos':
				return {
					...base,
					tolerance: cfg?.tolerance,
					vid_hash_duration: cfg?.vid_hash_duration,
					crop_detect: cfg?.crop_detect,
				};
			case 'similar-images':
				return {
					...base,
					hash_alg: cfg?.hash_alg,
					hash_size: cfg?.hash_size,
					resize_filter: cfg?.resize_filter,
					similarity: cfg?.similarity,
				};
			case 'same-music':
				return { ...base, music_check_type: cfg?.music_check_type };
			case 'broken-files':
				return { ...base, broken_file_types: cfg?.broken_file_types };
			case 'bad-extensions':
				return { ...base, include_files_without_extension: cfg?.include_files_without_extension };
			case 'bad-names':
				return {
					...base,
					bad_name_uppercase_extension: cfg?.bad_name_uppercase_extension,
					bad_name_emoji: cfg?.bad_name_emoji,
					bad_name_spaces: cfg?.bad_name_spaces,
					bad_name_non_ascii: cfg?.bad_name_non_ascii,
					bad_name_restricted_charset: cfg?.bad_name_restricted_charset,
					bad_name_allowed_chars: cfg?.bad_name_allowed_chars,
					bad_name_dedupe_non_alnum: cfg?.bad_name_dedupe_non_alnum,
				};
			default:
				return base;
		}
	}

	// ---- State loading ----
	async function loadState() {
		try {
			const state = await api.getState();
			backendState = state;
			includedDirs = state.directories.included;
			excludedDirs = state.directories.excluded;
			excludedItems = state.directories.excluded_items;

			try {
				const defaults = await api.getDefaults();
				defaultExcludedDirs = defaults.excluded_directories;
				if (excludedDirs.length === 0 && excludedItems === '') {
					excludedDirs = defaults.excluded_directories;
					excludedItems = defaults.excluded_items;
				}
			} catch (e) {
				console.error('Failed to load defaults:', e);
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
			scanProgress = null;
			scanError = '';
			scanId = '';
			checkedFiles = new SvelteSet();
			selectedFileSize = 0;
			return;
		}
		scanState = tool.status as typeof scanState;
		scanResults = tool.results ?? null;
		scanProgress = null;
		scanError = tool.error ?? '';
		scanId = tool.scan_id ?? '';
		checkedFiles = new SvelteSet(tool.checked_files ?? []);
		selectedFileSize = selectedFile && scanResults ? groupSizeFor(selectedFile) : 0;
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

	// ---- Modal & preview ----
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

	function updateBackendTool() {
		if (!backendState) return;
		backendState.tools[activeTool] = {
			status: scanState,
			results: scanResults ?? undefined,
			error: scanError || undefined,
			scan_id: scanId || undefined,
			checked_files: Array.from(checkedFiles),
		};
	}

	// ---- Polling ----
	async function poll() {
		if (!scanId) return;
		try {
			const res = await api.getScanStatus(scanId);
			scanError = '';
			if (res.status === 'completed') {
				scanState = 'completed';
				scanProgress = null;
				scanResults = res.results ?? null;
				updateBackendTool();
				clearInterval(intervalId);
			} else if (res.status === 'error') {
				scanState = 'error';
				scanProgress = null;
				scanError = res.error ?? 'Unknown error';
				updateBackendTool();
				clearInterval(intervalId);
			} else if (res.status === 'cancelled') {
				scanState = 'idle';
				scanProgress = null;
				scanResults = null;
				updateBackendTool();
				clearInterval(intervalId);
			} else if (res.status === 'not_found') {
				scanState = 'error';
				scanProgress = null;
				scanError = 'Scan not found';
				updateBackendTool();
				clearInterval(intervalId);
			} else if (res.status === 'running') {
				scanProgress = res.progress ?? null;
			}
		} catch (err) {
			scanState = 'error';
			scanProgress = null;
			scanError = err instanceof Error ? err.message : 'Failed to fetch status';
			updateBackendTool();
			clearInterval(intervalId);
		}
	}

	// ---- Actions ----
	async function handleDelete() {
		const files = Array.from(checkedFiles);
		if (files.length === 0) return;
		try {
			const res = await api.deleteFiles(activeTool, files);
			scanError = '';
			for (const path of res.deleted) checkedFiles.delete(path);
			selectedFile = null;
			selectedFileSize = 0;
			toolSelections[activeTool] = { path: null, size: 0 };
			removeFromResults(res.deleted);
			updateBackendTool();
			if (res.failed.length > 0) {
				scanError = `Failed to delete ${res.failed.length} file${res.failed.length === 1 ? '' : 's'}`;
			}
		} catch (err) {
			scanError = err instanceof Error ? err.message : 'Failed to delete files';
			updateBackendTool();
		}
	}

	async function handleFix() {
		const files = Array.from(checkedFiles);
		if (files.length === 0) return;
		try {
			const cfg = toolConfigs[activeTool];
			const res = await api.fixFiles({
				tool_id: activeTool,
				files,
				bad_name_uppercase_extension: cfg?.bad_name_uppercase_extension,
				bad_name_emoji: cfg?.bad_name_emoji,
				bad_name_spaces: cfg?.bad_name_spaces,
				bad_name_non_ascii: cfg?.bad_name_non_ascii,
				bad_name_restricted_charset: cfg?.bad_name_restricted_charset,
				bad_name_allowed_chars: cfg?.bad_name_allowed_chars,
				bad_name_dedupe_non_alnum: cfg?.bad_name_dedupe_non_alnum,
			});
			scanError = '';
			for (const path of res.fixed) checkedFiles.delete(path);
			selectedFile = null;
			selectedFileSize = 0;
			toolSelections[activeTool] = { path: null, size: 0 };
			removeFromResults(res.fixed);
			updateBackendTool();
			if (res.failed.length > 0) {
				scanError = `Failed to fix ${res.failed.length} file${res.failed.length === 1 ? '' : 's'}`;
			}
		} catch (err) {
			scanError = err instanceof Error ? err.message : 'Failed to fix files';
			updateBackendTool();
		}
	}

	async function startScan() {
		const payload = buildPayload();
		if (payload.directories.length === 0) {
			scanError = 'Please enter at least one directory.';
			return;
		}
		scanState = 'running';
		scanError = '';
		scanResults = null;
		scanProgress = null;
		selectedFile = null;
		selectedFileSize = 0;
		try {
			const res = await api.startScan(payload);
			scanId = res.id;
			updateBackendTool();
			poll();
			intervalId = setInterval(poll, 1000);
		} catch (err) {
			scanState = 'error';
			scanError = err instanceof Error ? err.message : 'Failed to start scan';
			updateBackendTool();
		}
	}

	async function cancelScan() {
		if (!scanId) return;
		try {
			await api.cancelScan(scanId);
			scanState = 'idle';
			scanProgress = null;
			scanResults = null;
			updateBackendTool();
			clearInterval(intervalId);
		} catch (err) {
			console.error('Failed to cancel scan:', err);
		}
	}

	function toggleSidebar() {
		sidebarCollapsed = !sidebarCollapsed;
	}
</script>

<div class="flex h-full w-full">
	<ToolSidebar {activeTool} collapsed={sidebarCollapsed} onChangeTool={switchTool} onToggleCollapse={toggleSidebar} />

	<div class="flex min-h-0 flex-1 flex-col overflow-hidden bg-bg">
		<ScanConfig
			bind:includedDirs
			bind:excludedDirs
			bind:excludedItems
			bind:activeTool
			bind:toolConfig={toolConfigs[activeTool]}
			{defaultExcludedDirs}
			{scanState}
			{scanResults}
			{checkedFiles}
			onStartScan={startScan}
			onCancelScan={cancelScan}
			onAddDir={openModal}
			onDelete={handleDelete}
			onFix={handleFix}
		/>

		<div class="flex min-h-0 flex-1 overflow-hidden">
			<ScanResults
				{scanState}
				{scanError}
				{scanResults}
				{scanProgress}
				onSelectFile={selectFile}
				{checkedFiles}
				{activeTool}
			/>

			{#if selectedFile}
				<FilePreview
					{selectedFile}
					{selectedFileSize}
					groupFiles={selectedFileGroup?.files ?? []}
					onClose={closePreview}
				/>
			{/if}
		</div>
	</div>
</div>

<DirectoryBrowserModal open={modalOpen} onClose={() => (modalOpen = false)} onSelect={handleModalSelect} />
