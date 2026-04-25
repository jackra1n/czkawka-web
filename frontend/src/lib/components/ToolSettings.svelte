<script lang="ts">
	import type { ToolConfig } from '$lib/api';

	let {
		activeTool,
		toolConfig = $bindable<ToolConfig>({})
	}: {
		activeTool: string;
		toolConfig: ToolConfig;
	} = $props();

	const IN = 'w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:outline-none focus:ring-1 focus:ring-accent';

	const HASH_ALGS = ['Mean', 'Gradient', 'Blockhash', 'VertGradient', 'DoubleGradient', 'Median'];
	const HASH_SIZES = [8, 16, 32, 64];
	const RESIZE_FILTERS = ['Lanczos3', 'Nearest', 'Triangle', 'Gaussian', 'CatmullRom'];
	const CROP_DETECTS = ['None', 'Letterbox', 'Motion'];
	const SEARCH_MODES = [
		{ value: 'biggest', label: 'Biggest' },
		{ value: 'smallest', label: 'Smallest' }
	];
	const BROKEN_TYPES = [
		{ key: 'pdf', label: 'PDF' },
		{ key: 'audio', label: 'Audio' },
		{ key: 'image', label: 'Image' },
		{ key: 'archive', label: 'Archive' },
		{ key: 'video', label: 'Video' }
	];
	const BAD_NAME_OPTS = [
		{ key: 'bad_name_uppercase_extension', label: 'Uppercase extension' },
		{ key: 'bad_name_emoji', label: 'Emoji' },
		{ key: 'bad_name_spaces', label: 'Spaces at ends' },
		{ key: 'bad_name_non_ascii', label: 'Non-ASCII' },
		{ key: 'bad_name_restricted_charset', label: 'Restricted charset' },
		{ key: 'bad_name_dedupe_non_alnum', label: 'Duplicate symbols' }
	];

	function u(cfg: Partial<ToolConfig>) {
		toolConfig = { ...toolConfig, ...cfg };
	}

	function brokenTypes(): string[] {
		return (toolConfig.broken_file_types ?? 'pdf,audio,image,archive,video').split(',').filter(Boolean);
	}

	function toggleBroken(key: string, checked: boolean) {
		const cur = brokenTypes();
		u({ broken_file_types: (checked ? [...cur, key] : cur.filter((k) => k !== key)).join(',') });
	}

	function isBroken(key: string) {
		return brokenTypes().includes(key);
	}

	function badNameDefault(key: string) {
		return key === 'bad_name_restricted_charset' || key === 'bad_name_dedupe_non_alnum' ? false : true;
	}
</script>

{#if activeTool === 'big-files'}
	<div class="flex flex-col gap-4">
		<div class="flex gap-4">
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="search-mode" class="text-xs font-medium text-text-muted">Search Mode</label>
				<select id="search-mode" value={toolConfig.search_mode ?? 'biggest'} onchange={(e) => u({ search_mode: e.currentTarget.value })} class={IN}>
					{#each SEARCH_MODES as m (m.value)}<option value={m.value}>{m.label}</option>{/each}
				</select>
			</div>
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="number-of-files" class="text-xs font-medium text-text-muted">Number of Files</label>
				<input id="number-of-files" type="number" min="1" max="10000" value={toolConfig.number_of_files ?? 50} oninput={(e) => u({ number_of_files: Number(e.currentTarget.value) })} class={IN} />
			</div>
		</div>
	</div>

{:else if activeTool === 'similar-videos'}
	<div class="flex flex-col gap-4">
		<div class="flex gap-4">
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="crop-detect" class="text-xs font-medium text-text-muted">Crop Detect</label>
				<select id="crop-detect" value={toolConfig.crop_detect ?? 'Letterbox'} onchange={(e) => u({ crop_detect: e.currentTarget.value })} class={IN}>
					{#each CROP_DETECTS as cd (cd)}<option value={cd}>{cd}</option>{/each}
				</select>
			</div>
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="vid-hash-duration" class="text-xs font-medium text-text-muted">Hash Duration (s)</label>
				<input id="vid-hash-duration" type="number" min="2" max="60" value={toolConfig.vid_hash_duration ?? 10} oninput={(e) => u({ vid_hash_duration: Number(e.currentTarget.value) })} class={IN} />
			</div>
		</div>
		<div class="flex flex-col gap-1.5">
			<label for="tolerance" class="text-xs font-medium text-text-muted">Tolerance</label>
			<div class="flex items-center gap-3">
				<span class="text-[10px] text-text-muted shrink-0 w-14 text-right">Strict</span>
				<input id="tolerance" type="range" min="0" max="20" step="1" value={toolConfig.tolerance ?? 5} oninput={(e) => u({ tolerance: Number(e.currentTarget.value) })} class="flex-1 accent-accent" />
				<span class="text-xs font-medium text-text w-6 text-center">{toolConfig.tolerance ?? 5}</span>
				<span class="text-[10px] text-text-muted shrink-0 w-12">Loose</span>
			</div>
		</div>
	</div>

{:else if activeTool === 'similar-images'}
	<div class="flex flex-col gap-4">
		<div class="flex gap-4">
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="hash-alg" class="text-xs font-medium text-text-muted">Hash Algorithm</label>
				<select id="hash-alg" value={toolConfig.hash_alg ?? 'Gradient'} onchange={(e) => u({ hash_alg: e.currentTarget.value })} class={IN}>
					{#each HASH_ALGS as a (a)}<option value={a}>{a}</option>{/each}
				</select>
			</div>
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="hash-size" class="text-xs font-medium text-text-muted">Hash Size</label>
				<select id="hash-size" value={toolConfig.hash_size ?? 16} onchange={(e) => u({ hash_size: Number(e.currentTarget.value) })} class={IN}>
					{#each HASH_SIZES as s (s)}<option value={s}>{s}</option>{/each}
				</select>
			</div>
		</div>
		<div class="flex flex-col gap-1.5">
			<label for="resize-filter" class="text-xs font-medium text-text-muted">Resize Algorithm</label>
			<select id="resize-filter" value={toolConfig.resize_filter ?? 'Lanczos3'} onchange={(e) => u({ resize_filter: e.currentTarget.value })} class={IN}>
				{#each RESIZE_FILTERS as f (f)}<option value={f}>{f}</option>{/each}
			</select>
		</div>
		<div class="flex flex-col gap-1.5">
			<label for="similarity" class="text-xs font-medium text-text-muted">Similarity</label>
			<div class="flex items-center gap-3">
				<span class="text-[10px] text-text-muted shrink-0 w-14 text-right">Very high</span>
				<input id="similarity" type="range" min="0" max="40" step="1" value={toolConfig.similarity ?? 5} oninput={(e) => u({ similarity: Number(e.currentTarget.value) })} class="flex-1 accent-accent" />
				<span class="text-xs font-medium text-text w-6 text-center">{toolConfig.similarity ?? 5}</span>
				<span class="text-[10px] text-text-muted shrink-0 w-12">Minimal</span>
			</div>
		</div>
	</div>

{:else if activeTool === 'same-music'}
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-1.5">
			<label for="music-check-type" class="text-xs font-medium text-text-muted">Comparison Method</label>
			<select id="music-check-type" value={toolConfig.music_check_type ?? 'tags'} onchange={(e) => u({ music_check_type: e.currentTarget.value })} class={IN}>
				<option value="tags">Tags (metadata)</option>
				<option value="content">Content (audio fingerprint)</option>
			</select>
		</div>
	</div>

{:else if activeTool === 'broken-files'}
	<div class="flex flex-col gap-3">
		<span class="text-xs font-medium text-text-muted">File types to check</span>
		<div class="flex flex-wrap gap-4">
			{#each BROKEN_TYPES as opt (opt.key)}
				<label class="flex items-center gap-1.5 text-sm text-text cursor-pointer">
					<input type="checkbox" checked={isBroken(opt.key)} onchange={(e) => toggleBroken(opt.key, e.currentTarget.checked)} />
					{opt.label}
				</label>
			{/each}
		</div>
	</div>

{:else if activeTool === 'bad-extensions'}
	<div class="flex flex-col gap-3">
		<label class="flex items-center gap-2 text-sm text-text cursor-pointer">
			<input type="checkbox" checked={toolConfig.include_files_without_extension ?? false} onchange={(e) => u({ include_files_without_extension: e.currentTarget.checked })} />
			Include files without extension
		</label>
	</div>

{:else if activeTool === 'bad-names'}
	<div class="flex flex-col gap-3">
		<span class="text-xs font-medium text-text-muted">Name issues to check</span>
		<div class="flex flex-wrap gap-4">
			{#each BAD_NAME_OPTS as opt (opt.key)}
				<label class="flex items-center gap-1.5 text-sm text-text cursor-pointer">
					<input type="checkbox" checked={toolConfig[opt.key] ?? badNameDefault(opt.key)} onchange={(e) => u({ [opt.key]: e.currentTarget.checked })} />
					{opt.label}
				</label>
			{/each}
		</div>
		{#if toolConfig.bad_name_restricted_charset}
			<div class="flex flex-col gap-1.5">
				<label for="bad-name-allowed-chars" class="text-xs font-medium text-text-muted">Allowed characters</label>
				<input id="bad-name-allowed-chars" type="text" value={toolConfig.bad_name_allowed_chars ?? '_- .'} oninput={(e) => u({ bad_name_allowed_chars: e.currentTarget.value })} class={IN} />
				<span class="text-xs text-text-muted">Characters allowed in filenames besides alphanumeric.</span>
			</div>
		{/if}
	</div>
{/if}
