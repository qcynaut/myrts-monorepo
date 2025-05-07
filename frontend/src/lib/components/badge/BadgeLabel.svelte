<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import Times from '../icon/Times.svelte';
	import { createEventDispatcher } from 'svelte';

	export let name: string = '';
	export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
	export let rounded: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'full';
	export let bg = 'primary-600';
	export let padding: string = 'p-2';

	const dispatch = createEventDispatcher();
	const textSize = twMerge(`text-${size}`);
	const classes = twMerge(`rounded-${rounded} bg-${bg} ${padding}`);

	const close = (e: Event) => {
		e.stopPropagation();
		dispatch('close');
	};
</script>

<div class="inline-block">
	<div class="text-white flex items-center gap-4 {classes}">
		<p class={textSize}>{name}</p>
		<button class="flex items-center" on:click={close}
			><Times class="fill-white {size == 'sm' || size == 'md' ? 'w-4 h-4' : 'w-6 h-6'}" /></button
		>
	</div>
</div>
