export type VirtualListItem<T> = {
	item: T;
	index: number;
	top: number;
	height: number;
};

export function computeVirtualLayout<T>(
	listItems: T[],
	getHeight: (item: T) => number,
): { items: VirtualListItem<T>[]; totalHeight: number } {
	let currentTop = 0;
	const items: VirtualListItem<T>[] = [];
	for (let i = 0; i < listItems.length; i++) {
		const item = listItems[i];
		const height = getHeight(item);
		items.push({
			item,
			index: i,
			top: currentTop,
			height,
		});
		currentTop += height;
	}
	return {
		items,
		totalHeight: currentTop,
	};
}

export function findVisibleRange<T>(
	items: VirtualListItem<T>[],
	viewTop: number,
	viewBottom: number,
): { start: number; end: number } {
	if (items.length === 0) return { start: 0, end: 0 };

	let low = 0;
	let high = items.length - 1;
	let start = 0;

	while (low <= high) {
		const mid = Math.floor((low + high) / 2);
		if (items[mid].top + items[mid].height >= viewTop) {
			start = mid;
			high = mid - 1;
		} else {
			low = mid + 1;
		}
	}

	low = 0;
	high = items.length - 1;
	let end = items.length - 1;

	while (low <= high) {
		const mid = Math.floor((low + high) / 2);
		if (items[mid].top <= viewBottom) {
			end = mid;
			low = mid + 1;
		} else {
			high = mid - 1;
		}
	}

	return { start, end };
}
