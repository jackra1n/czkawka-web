import type { ToolConfig } from '$lib/api';

export const DEFAULT_TOOL_CONFIGS: Record<string, ToolConfig> = {
	duplicates: {},
	'empty-files': {},
	'empty-folders': {},
	'big-files': { number_of_files: 50, search_mode: 'biggest' },
	'similar-images': {
		hash_alg: 'Gradient',
		hash_size: 16,
		resize_filter: 'Lanczos3',
		similarity: 5,
	},
	'similar-videos': { tolerance: 5, vid_hash_duration: 10, crop_detect: 'Letterbox' },
	'same-music': { music_check_type: 'tags' },
	'invalid-symlinks': {},
	'broken-files': { broken_file_types: 'pdf,audio,image,archive,video' },
	'bad-extensions': { include_files_without_extension: false },
	'exif-remover': {},
	'bad-names': {
		bad_name_uppercase_extension: true,
		bad_name_emoji: true,
		bad_name_spaces: true,
		bad_name_non_ascii: true,
		bad_name_restricted_charset: false,
		bad_name_allowed_chars: '_- .',
		bad_name_dedupe_non_alnum: false,
	},
	temporary: {},
};
