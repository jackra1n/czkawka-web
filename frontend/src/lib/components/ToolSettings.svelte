<script lang="ts">
	import type { BadNamesConfig, ToolConfig } from '$lib/api';
	import Select from './ui/Select.svelte';
	import NumberInput from './ui/NumberInput.svelte';
	import Checkbox from './ui/Checkbox.svelte';
	import Range from './ui/Range.svelte';

	let {
		activeTool,
		toolConfig = $bindable<ToolConfig>({}),
	}: {
		activeTool: string;
		toolConfig: ToolConfig;
	} = $props();

	const HASH_ALGS = ['Mean', 'Gradient', 'Blockhash', 'VertGradient', 'DoubleGradient', 'Median'];
	const HASH_SIZES = [8, 16, 32, 64];
	const RESIZE_FILTERS = ['Lanczos3', 'Nearest', 'Triangle', 'Gaussian', 'CatmullRom'];

	const SEARCH_MODE_OPTS = [
		{ value: 'biggest', label: 'Biggest' },
		{ value: 'smallest', label: 'Smallest' },
	];
	const HASH_ALG_OPTS = HASH_ALGS.map((a) => ({ value: a, label: a }));
	const HASH_SIZE_OPTS = HASH_SIZES.map((s) => ({ value: String(s), label: String(s) }));
	const RESIZE_FILTER_OPTS = RESIZE_FILTERS.map((f) => ({ value: f, label: f }));
	const MUSIC_OPTS = [
		{ value: 'tags', label: 'Tags (metadata)' },
		{ value: 'content', label: 'Content (audio fingerprint)' },
	];

	const BROKEN_TYPES = [
		{ key: 'pdf', label: 'PDF' },
		{ key: 'audio', label: 'Audio' },
		{ key: 'image', label: 'Image' },
		{ key: 'archive', label: 'Archive' },
		{ key: 'video', label: 'Video' },
	];
	const BAD_NAME_OPTS: { key: keyof BadNamesConfig; label: string }[] = [
		{ key: 'bad_name_uppercase_extension', label: 'Uppercase extension' },
		{ key: 'bad_name_emoji', label: 'Emoji' },
		{ key: 'bad_name_spaces', label: 'Spaces at ends' },
		{ key: 'bad_name_non_ascii', label: 'Non-ASCII' },
		{ key: 'bad_name_restricted_charset', label: 'Restricted charset' },
		{ key: 'bad_name_dedupe_non_alnum', label: 'Duplicate symbols' },
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

	function badNameDefault(key: keyof BadNamesConfig) {
		return key === 'bad_name_restricted_charset' || key === 'bad_name_dedupe_non_alnum' ? false : true;
	}
</script>

{#if activeTool === 'big-files'}
	<div class="flex flex-col gap-4">
		<div class="flex gap-4">
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="search-mode" class="text-xs font-medium text-text-muted">Search Mode</label>
				<Select
					id="search-mode"
					value={toolConfig.search_mode ?? 'biggest'}
					onchange={(v) => u({ search_mode: v })}
					options={SEARCH_MODE_OPTS}
				/>
			</div>
			<div class="flex flex-1 flex-col gap-1.5">
				<label for="number-of-files" class="text-xs font-medium text-text-muted">Number of Files</label>
				<NumberInput
					id="number-of-files"
					min={1}
					max={10000}
					value={toolConfig.number_of_files ?? 50}
					onchange={(v) => u({ number_of_files: v })}
				/>
			</div>
		</div>
	</div>
{:else if activeTool === 'similar-videos'}
	<div class="flex flex-col gap-4">
		<div class="flex flex-wrap gap-4">
			<div class="flex min-w-30 flex-1 flex-col gap-1.5">
				<Checkbox
					checked={toolConfig.crop_detect ?? true}
					onchange={(checked) => u({ crop_detect: checked })}
				>
					Detect cropped videos
				</Checkbox>
			</div>
			<div class="flex min-w-30 flex-1 flex-col gap-1.5">
				<label for="vid-hash-duration" class="text-xs font-medium text-text-muted">Hash Duration (s)</label>
				<NumberInput
					id="vid-hash-duration"
					min={2}
					max={60}
					value={toolConfig.vid_hash_duration ?? 10}
					onchange={(v) => u({ vid_hash_duration: v })}
				/>
			</div>
			<div class="flex min-w-60 flex-2 flex-col gap-1.5">
				<label for="tolerance" class="text-xs font-medium text-text-muted">Tolerance</label>
				<div class="flex items-center gap-3 py-2">
					<span class="shrink-0 text-xs text-text-muted">Strict</span>
					<Range
						id="tolerance"
						min={0}
						max={20}
						step={1}
						value={toolConfig.tolerance ?? 5}
						onchange={(v) => u({ tolerance: v })}
					/>
					<span class="shrink-0 text-xs font-medium text-text">{toolConfig.tolerance ?? 5}</span>
					<span class="shrink-0 text-xs text-text-muted">Loose</span>
				</div>
			</div>
		</div>
	</div>
{:else if activeTool === 'similar-images'}
	<div class="flex flex-col gap-4">
		<div class="flex flex-wrap gap-4">
			<div class="flex min-w-30 flex-1 flex-col gap-1.5">
				<label for="hash-alg" class="text-xs font-medium text-text-muted">Hash Algorithm</label>
				<Select
					id="hash-alg"
					value={toolConfig.hash_alg ?? 'Gradient'}
					onchange={(v) => u({ hash_alg: v })}
					options={HASH_ALG_OPTS}
				/>
			</div>
			<div class="flex min-w-30 flex-1 flex-col gap-1.5">
				<label for="hash-size" class="text-xs font-medium text-text-muted">Hash Size</label>
				<Select
					id="hash-size"
					value={String(toolConfig.hash_size ?? 16)}
					onchange={(v) => u({ hash_size: Number(v) })}
					options={HASH_SIZE_OPTS}
				/>
			</div>
			<div class="flex min-w-30 flex-1 flex-col gap-1.5">
				<label for="resize-filter" class="text-xs font-medium text-text-muted">Resize Algorithm</label>
				<Select
					id="resize-filter"
					value={toolConfig.resize_filter ?? 'Lanczos3'}
					onchange={(v) => u({ resize_filter: v })}
					options={RESIZE_FILTER_OPTS}
				/>
			</div>
			<div class="flex min-w-60 flex-2 flex-col gap-1.5">
				<label for="similarity" class="text-xs font-medium text-text-muted">Similarity</label>
				<div class="flex items-center gap-3 py-2">
					<span class="shrink-0 text-xs text-text-muted">High</span>
					<Range
						id="similarity"
						min={0}
						max={40}
						step={1}
						value={toolConfig.similarity ?? 5}
						onchange={(v) => u({ similarity: v })}
					/>
					<span class="shrink-0 text-xs font-medium text-text">{toolConfig.similarity ?? 5}</span>
					<span class="shrink-0 text-xs text-text-muted">Minimal</span>
				</div>
			</div>
		</div>
	</div>
{:else if activeTool === 'same-music'}
	<div class="flex flex-col gap-4">
		<div class="flex flex-col gap-1.5">
			<label for="music-check-type" class="text-xs font-medium text-text-muted">Comparison Method</label>
			<Select
				id="music-check-type"
				value={toolConfig.music_check_type ?? 'tags'}
				onchange={(v) => u({ music_check_type: v })}
				options={MUSIC_OPTS}
			/>
		</div>
	</div>
{:else if activeTool === 'broken-files'}
	<div class="flex flex-col gap-3">
		<span class="text-xs font-medium text-text-muted">File types to check</span>
		<div class="flex flex-wrap gap-4">
			{#each BROKEN_TYPES as opt (opt.key)}
				<Checkbox checked={isBroken(opt.key)} onchange={(checked) => toggleBroken(opt.key, checked)}>
					{opt.label}
				</Checkbox>
			{/each}
		</div>
	</div>
{:else if activeTool === 'bad-extensions'}
	<div class="flex flex-col gap-3">
		<Checkbox
			checked={toolConfig.include_files_without_extension ?? false}
			onchange={(checked) => u({ include_files_without_extension: checked })}
		>
			Include files without extension
		</Checkbox>
	</div>
{:else if activeTool === 'bad-names'}
	<div class="flex flex-col gap-3">
		<span class="text-xs font-medium text-text-muted">Name issues to check</span>
		<div class="flex flex-wrap gap-4">
			{#each BAD_NAME_OPTS as opt (opt.key)}
				<Checkbox
					checked={(toolConfig[opt.key] as boolean) ?? badNameDefault(opt.key)}
					onchange={(checked) => u({ [opt.key]: checked })}
				>
					{opt.label}
				</Checkbox>
			{/each}
		</div>
		{#if toolConfig.bad_name_restricted_charset}
			<div class="flex flex-col gap-1.5">
				<label for="bad-name-allowed-chars" class="text-xs font-medium text-text-muted">Allowed characters</label>
				<input
					id="bad-name-allowed-chars"
					type="text"
					value={toolConfig.bad_name_allowed_chars ?? '_- .'}
					oninput={(e) => u({ bad_name_allowed_chars: e.currentTarget.value })}
					class="w-full rounded-md border border-border bg-bg px-3 py-2 text-sm text-text focus:border-accent focus:ring-1 focus:ring-accent focus:outline-none"
				/>
				<span class="text-xs text-text-muted">Characters allowed in filenames besides alphanumeric.</span>
			</div>
		{/if}
	</div>
{/if}
