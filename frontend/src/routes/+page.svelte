<script lang="ts">
	import { api, type ScanResults } from '$lib/api';
	import {
		Copy,
		FolderOpen,
		BarChart3,
		FileX,
		Clock,
		Images,
		Video,
		Music,
		Link,
		AlertTriangle,
		FileCode,
		Camera,
		Type,
		Search,
		ChevronDown,
		File,
		X,
		Lock
	} from 'lucide-svelte';
	import { onMount, onDestroy } from 'svelte';

	let directories = $state('');
	let excludeDirectories = $state('');
	let minFileSize = $state(8192);

	let scanState = $state<'idle' | 'running' | 'completed' | 'error'>('idle');
	let scanError = $state('');
	let scanResults = $state<ScanResults | null>(null);
	let scanId = $state('');

	let expandedGroups = $state<Set<number>>(new Set());
	let selectedFile = $state<string | null>(null);
	let selectedFileSize = $state(0);

	let intervalId: ReturnType<typeof setInterval>;

	const tools = [
		{ id: 'duplicates', label: 'Duplicate Files', icon: Copy, disabled: false },
		{ id: 'empty-folders', label: 'Empty Folders', icon: FolderOpen, disabled: true },
		{ id: 'big-files', label: 'Big Files', icon: BarChart3, disabled: true },
		{ id: 'empty-files', label: 'Empty Files', icon: FileX, disabled: true },
		{ id: 'temporary', label: 'Temporary Files', icon: Clock, disabled: true },
		{ id: 'similar-images', label: 'Similar Images', icon: Images, disabled: true },
		{ id: 'similar-videos', label: 'Similar Videos', icon: Video, disabled: true },
		{ id: 'same-music', label: 'Same Music', icon: Music, disabled: true },
		{ id: 'invalid-symlinks', label: 'Invalid Symlinks', icon: Link, disabled: true },
		{ id: 'broken-files', label: 'Broken Files', icon: AlertTriangle, disabled: true },
		{ id: 'bad-extensions', label: 'Bad Extensions', icon: FileCode, disabled: true },
		{ id: 'exif-remover', label: 'Exif Remover', icon: Camera, disabled: true },
		{ id: 'bad-names', label: 'Bad Names', icon: Type, disabled: true }
	];

	let activeTool = $state('duplicates');

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
	}

	function formatDuration(ms: number): string {
		if (ms < 1000) return ms + ' ms';
		return (ms / 1000).toFixed(2) + ' s';
	}

	function toggleGroup(index: number) {
		const next = new Set(expandedGroups);
		if (next.has(index)) {
			next.delete(index);
		} else {
			next.add(index);
		}
		expandedGroups = next;
	}

	function selectFile(file: string, size: number) {
		selectedFile = file;
		selectedFileSize = size;
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
		} catch (err: any) {
			scanState = 'error';
			scanError = err.message ?? 'Failed to fetch status';
			clearInterval(intervalId);
		}
	}

	async function startScan() {
		const dirs = directories
			.split('\n')
			.map((s) => s.trim())
			.filter(Boolean);

		if (dirs.length === 0) {
			scanError = 'Please enter at least one directory.';
			return;
		}

		const excluded = excludeDirectories
			.split('\n')
			.map((s) => s.trim())
			.filter(Boolean);

		scanState = 'running';
		scanError = '';
		scanResults = null;
		selectedFile = null;
		expandedGroups = new Set();

		try {
			const res = await api.startScan({
				directories: dirs,
				exclude_directories: excluded.length > 0 ? excluded : undefined,
				min_file_size: minFileSize
			});
			scanId = res.id;
			poll();
			intervalId = setInterval(poll, 1000);
		} catch (err: any) {
			scanState = 'error';
			scanError = err.message ?? 'Failed to start scan';
		}
	}

	onDestroy(() => {
		clearInterval(intervalId);
	});
</script>

<div class="flex h-full w-full">
	<!-- Sidebar -->
	<aside class="flex w-56 shrink-0 flex-col overflow-y-auto border-r border-border bg-surface">
		<div class="flex flex-col gap-0.5 p-2">
			{#each tools as tool}
				<button
					class="flex items-center gap-2.5 rounded-md px-3 py-2 text-left text-sm transition-colors"
					class:bg-surface-raised={tool.id === activeTool}
					class:text-text={tool.id === activeTool}
					class:text-text-muted={tool.id !== activeTool}
					class:hover:bg-surface-raised={!tool.disabled}
					class:hover:text-text={!tool.disabled}
					class:opacity-40={tool.disabled}
					class:cursor-not-allowed={tool.disabled}
					disabled={tool.disabled}
					onclick={() => {
						if (!tool.disabled) activeTool = tool.id;
					}}
				>
					{#if tool.disabled}
						<Lock class="h-3.5 w-3.5 shrink-0 opacity-60" />
					{:else}
						<tool.icon class="h-4 w-4 shrink-0" />
					{/if}
					<span class="truncate">{tool.label}</span>
				</button>
			{/each}
		</div>
	</aside>

	<!-- Main content -->
	<div class="flex flex-1 flex-col min-h-0 overflow-hidden bg-bg">
		<!-- Top config bar -->
		<div class="shrink-0 border-b border-border bg-surface p-4">
			<div class="flex gap-4">
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="dirs" class="text-xs font-medium text-text-muted">Included directories</label>
					<textarea
						id="dirs"
						bind:value={directories}
						placeholder="/home/user/Downloads&#10;/home/user/Desktop"
						rows={2}
						class="w-full resize-none rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					></textarea>
				</div>
				<div class="flex flex-1 flex-col gap-1.5">
					<label for="exclude" class="text-xs font-medium text-text-muted">Excluded directories</label>
					<textarea
						id="exclude"
						bind:value={excludeDirectories}
						placeholder="/home/user/Downloads/temp"
						rows={2}
						class="w-full resize-none rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					></textarea>
				</div>
			</div>
			<div class="mt-3 flex items-end gap-3">
				<div class="flex flex-col gap-1.5">
					<label for="min-size" class="text-xs font-medium text-text-muted">Min size (bytes)</label>
					<input
						id="min-size"
						type="number"
						bind:value={minFileSize}
						min={0}
						step={1024}
						class="w-32 rounded-md border border-border bg-bg px-3 py-2 text-sm text-text placeholder:text-text-muted focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent"
					/>
				</div>
				<button
					onclick={startScan}
					disabled={scanState === 'running'}
					class="inline-flex items-center gap-2 rounded-md bg-accent px-4 py-2 text-sm font-semibold text-white transition-colors hover:bg-accent-hover focus:outline-none focus:ring-2 focus:ring-accent focus:ring-offset-2 focus:ring-offset-surface disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{#if scanState === 'running'}
						<span class="inline-block h-4 w-4 animate-spin rounded-full border-2 border-white/30 border-t-white"></span>
						Scanning…
					{:else}
						<Search class="h-4 w-4" />
						Search
					{/if}
				</button>
			</div>
		</div>

		<!-- Results area -->
		<div class="flex flex-1 min-h-0 overflow-hidden">
			<div class="flex flex-1 flex-col min-h-0 overflow-auto">
				{#if scanState === 'idle' && !scanResults}
					<div class="flex flex-1 flex-col items-center justify-center gap-3 text-text-muted">
						<Search class="h-10 w-10 opacity-30" />
						<p class="text-sm">Enter directories and click Search to begin</p>
					</div>
				{:else if scanState === 'running'}
					<div class="flex flex-1 flex-col items-center justify-center gap-4">
						<div class="h-8 w-8 animate-spin rounded-full border-2 border-border border-t-accent"></div>
						<p class="text-sm text-text-muted">Scanning for duplicates…</p>
					</div>
				{:else if scanState === 'error'}
					<div class="flex flex-1 flex-col items-center justify-center gap-3 p-8">
						<div class="rounded-lg border border-danger/30 bg-danger/10 px-4 py-3 text-sm text-danger max-w-md w-full text-center">
							{scanError}
						</div>
					</div>
				{:else if scanResults}
					<!-- Stats bar -->
					<div class="flex items-center gap-6 border-b border-border px-4 py-2 text-xs text-text-muted">
						<span>Groups: <strong class="text-text">{scanResults.total_duplicate_groups}</strong></span>
						<span>Files: <strong class="text-text">{scanResults.total_duplicate_files}</strong></span>
						<span>Wasted: <strong class="text-text">{formatBytes(scanResults.wasted_space_bytes)}</strong></span>
						<span>Duration: <strong class="text-text">{formatDuration(scanResults.scanning_time_ms)}</strong></span>
					</div>

					{#if scanResults.groups.length === 0}
						<div class="flex flex-1 flex-col items-center justify-center text-sm text-text-muted">
							No duplicates found.
						</div>
					{:else}
						<!-- Table -->
						<div class="flex flex-col">
							<div class="grid grid-cols-[120px_1fr_80px_40px] gap-4 border-b border-border px-4 py-2 text-xs font-medium text-text-muted uppercase tracking-wider">
								<div>Size</div>
								<div>Hash</div>
								<div class="text-right">Files</div>
								<div></div>
							</div>
							{#each scanResults.groups as group, i (group.hash)}
								<div class="border-b border-border">
									<button
										class="grid w-full grid-cols-[120px_1fr_80px_40px] items-center gap-4 px-4 py-3 text-left transition-colors hover:bg-surface-raised"
										onclick={() => toggleGroup(i)}
									>
										<span class="text-sm font-medium text-text">{formatBytes(group.size)}</span>
										<span class="truncate font-mono text-xs text-text-muted">{group.hash}</span>
										<span class="text-right text-sm text-text-muted">{group.files.length}</span>
										<div class="flex justify-center">
										<ChevronDown
											class="h-4 w-4 text-text-muted transition-transform duration-200 {expandedGroups.has(i) ? 'rotate-180' : ''}"
										/>
										</div>
									</button>

									{#if expandedGroups.has(i)}
										<div class="bg-surface/40">
											{#each group.files as file}
												<button
													class="flex w-full items-center gap-3 px-4 py-2 pl-8 text-left text-sm text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
													onclick={() => selectFile(file, group.size)}
												>
													<File class="h-3.5 w-3.5 shrink-0 opacity-60" />
													<span class="truncate font-mono text-xs">{file}</span>
												</button>
											{/each}
										</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				{/if}
			</div>

			<!-- Preview panel -->
			{#if selectedFile}
				<aside class="flex w-72 shrink-0 flex-col border-l border-border bg-surface">
					<div class="flex items-center justify-between border-b border-border px-4 py-3">
						<span class="text-sm font-medium">Preview</span>
						<button
							onclick={() => (selectedFile = null)}
							class="rounded p-1 text-text-muted transition-colors hover:bg-surface-raised hover:text-text"
						>
							<X class="h-4 w-4" />
						</button>
					</div>
					<div class="flex flex-1 flex-col items-center gap-6 p-6">
						<div class="flex h-32 w-32 items-center justify-center rounded-lg border border-border bg-bg">
							<File class="h-12 w-12 text-text-muted opacity-40" />
						</div>
						<div class="w-full space-y-4">
							<div>
								<p class="mb-1 text-xs font-medium text-text-muted">Path</p>
								<p class="break-all text-xs font-mono leading-relaxed text-text">{selectedFile}</p>
							</div>
							<div>
								<p class="mb-1 text-xs font-medium text-text-muted">Size</p>
								<p class="text-sm text-text">{formatBytes(selectedFileSize)}</p>
							</div>
						</div>
					</div>
				</aside>
			{/if}
		</div>
	</div>
</div>
