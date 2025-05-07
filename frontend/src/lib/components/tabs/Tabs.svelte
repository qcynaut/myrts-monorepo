<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import Button from '../button/Button.svelte';
	import type { NameValue } from '$lib/types/components';

	export let items: NameValue<any>[] = [];
	export let active: any;
	export let containerClass: string = '';
	export let full: boolean = false;
</script>

<div class="{twMerge(containerClass)} flex items-center rounded-md">
	{#each items as item}
		<Button
			on:click={() => (active = item.value)}
			class="p-6 md:px-12 rounded-md {active == item.value
				? 'bg-primary-600 text-white'
				: 'bg-none text-black'} {full && 'w-full'}"
		>
			{item.name}
		</Button>
	{/each}
</div>
