<script lang="ts">
	import BreadCrumb from '../breadcrumb/BreadCrumb.svelte';
	import type { BreadCrumbItem } from '../breadcrumb/types';

	export let title: string;
	export let subtitle: string = '';
	export let breadcrumbs: BreadCrumbItem[] = [];
</script>

<div class="relative w-full h-screen flex flex-col overflow-y-auto bg-gray-100">
	<div class="py-8 px-4 bg-white flex justify-between items-center">
		<div>
			<h1 class="text-2xl font-semibold">{title}</h1>
			{#if breadcrumbs.length > 0}
				<BreadCrumb items={breadcrumbs} />
			{:else}
				<p class="text-gray-400">{subtitle}</p>
			{/if}
		</div>
		<slot name="endItem" />
	</div>
	<slot />
</div>
