<script lang="ts">
	import { getAudioContext } from '$lib/context/audio';
	import type { Writable } from 'svelte/store';
	import PlayerSlider from './PlayerSlider.svelte';
	import Icons from './Icons.svelte';
	import { Icon } from '$lib/const/icon';

	const { volume, muted } = getAudioContext();

	const toggle = (store: Writable<boolean>) => {
		store.update((s) => !s);
	};

	let volumePercentage = 100;
	$: $volume = volumePercentage / 100;
</script>

<div class="volume flex justify-center relative">
	<button on:click={() => toggle(muted)}>
		{#if $muted}
			<Icons name={Icon.VolumeMute} class=" text-primary-300" />
		{:else}
			<Icons name={Icon.VolumeUp} class="text-primary-300" />
		{/if}
	</button>

	<div class="volume-control w-16 origin-left -rotate-90 absolute -top-1 left-[50%]">
		<PlayerSlider bind:value={volumePercentage} max={100} step={1} />
	</div>
</div>
