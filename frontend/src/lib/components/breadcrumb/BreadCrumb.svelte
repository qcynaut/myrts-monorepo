<script lang="ts">
	import type { BreadCrumbItem } from './types';

	export let items: BreadCrumbItem[] = [];
</script>

<div class="flex gap-4 items-center">
	{#each items as item, i}
		{#if i !== items.length - 1}
			<a class="text-gray-500 hover:text-primary-600" href={item.url}>{item.name}</a>
			<span class="w-2 h-2 bg-gray-300 rounded-full" />
		{/if}
	{/each}
	<span class="text-gray-700">{items[items.length - 1].name}</span>
</div>
