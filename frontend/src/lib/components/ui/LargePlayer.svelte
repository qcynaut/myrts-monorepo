<script lang="ts">
	import { getAudioContext } from '$lib/context/audio';
	import type { Writable } from 'svelte/store';
	import PlayerSlider from './PlayerSlider.svelte';
	import Icons from './Icons.svelte';
	import { Icon } from '$lib/const/icon';

	export let playerPaused: boolean;
	export let seek: boolean;
	export let next: () => void = () => {};
	export let prev: () => void = () => {};
	export let close: () => void = () => {};

	const PLAYBACK_SPEEDS = [1, 1.25, 1.5, 1.75, 2, 0.25, 0.5, 0.75];

	const { playing, playbackRate, paused, seekBy, currentTime, duration, muted, volume } =
		getAudioContext();

	let speedIndex = 0;

	const handlePlaybackSpeedClick = () => {
		$playbackRate = PLAYBACK_SPEEDS[++speedIndex % PLAYBACK_SPEEDS.length];
	};

	const toggle = (store: Writable<boolean>) => {
		store.update((s) => !s);
		playerPaused = !playerPaused;
	};

	const toHHMMSS = (sec_num: number) => {
		if (isNaN(sec_num)) {
			return NaN;
		}

		sec_num = Math.floor(sec_num);
		const hours = Math.floor(sec_num / 3600);
		const minutes = Math.floor(sec_num / 60) % 60;
		const seconds = sec_num % 60;

		return [hours, minutes, seconds]
			.map((v) => (v < 10 ? '0' + v : v))
			.filter((v, i) => v !== '00' || i > 0)
			.join(':');
	};

	paused.subscribe((v) => {
		playerPaused = v;
		if ($currentTime == $duration) {
			next();
		}
	});

	let volumePercentage = 100;
	$: $volume = volumePercentage / 100;

	$: {
		if ($paused !== playerPaused) {
			$paused = playerPaused;
		}
	}

	$: {
		if (seek) seekBy(-1 * ($duration - $currentTime));
		seek = false;
	}
</script>

<div
	class="w-full p-4 flex flex-row gap-2 justify-between md:justify-center items-center shadow-md bg-primary-600 shadow-primary-500/10 rounded-xl"
>
	<div class="flex items-center space-x-3">
		<button on:click={close}>
			<Icons name={Icon.Times} class="text-primary-300" />
		</button>
		<button on:click={prev}>
			<Icons name={Icon.Backward} class="text-primary-300" />
		</button>
		<button on:click={() => (playerPaused = !playerPaused)} class="text-primary-300">
			{#if $playing}
				<Icons name={Icon.Pause} />
			{:else}
				<Icons name={Icon.Play} />
			{/if}
		</button>

		<button on:click={next}>
			<Icons name={Icon.Forward} class="text-primary-300" />
		</button>

		<button
			class="w-6 h-4 flex items-center justify-center rounded-full bg-primary-300"
			on:click={handlePlaybackSpeedClick}
		>
			<span class="text-[8px] font-semibold text-white">{$playbackRate}x</span>
		</button>
	</div>

	<div class="w-full hidden md:flex items-center space-x-2">
		<span class="text-sm text-primary-800">{toHHMMSS($currentTime)}</span>
		<PlayerSlider max={$duration} bind:value={$currentTime} />
		<span class="text-sm text-primary-800">{toHHMMSS($duration)}</span>
		<button on:click={() => toggle(muted)}>
			{#if $muted}
				<Icons name={Icon.VolumeMute} class=" text-primary-300" />
			{:else}
				<Icons name={Icon.VolumeUp} class="text-primary-300" />
			{/if}
		</button>
	</div>
	<span class="w-24"><PlayerSlider bind:value={volumePercentage} max={100} step={1} /></span>
</div>

<style lang="scss">
	:global(.icon-4) {
		@apply w-4 h-4;
	}
</style>
