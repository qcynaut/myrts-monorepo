<script lang="ts">
	import { getAudioContext } from '$lib/context/audio';
	import type { Writable } from 'svelte/store';
	import PlayerSlider from './PlayerSlider.svelte';
	import Icons from './Icons.svelte';
	import { Icon } from '$lib/const/icon';
	import VolumeControl from './VolumeControl.svelte';

	const SEEK_SECONDS = 10;
	const PLAYBACK_SPEEDS = [1, 1.25, 1.5, 1.75, 2, 0.25, 0.5, 0.75];

	const { playing, playbackRate, paused, repeat, seekBy, currentTime, duration } =
		getAudioContext();

	let speedIndex = 0;

	const handlePlaybackSpeedClick = () => {
		$playbackRate = PLAYBACK_SPEEDS[++speedIndex % PLAYBACK_SPEEDS.length];
	};

	const toggle = (store: Writable<boolean>) => {
		store.update((s) => !s);
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
</script>

<div
	class="h-40 w-[320px] p-8 flex flex-col justify-center items-center border border-gray-100 shadow-md shadow-primary-500/10 rounded-xl"
>
	<div class="flex items-center space-x-3">
		<button on:click={() => toggle(repeat)}>
			<Icons name={Icon.RedoAlt} class={`${$repeat ? 'text-primary-500' : 'text-primary-300'}`} />
		</button>

		<button on:click={() => seekBy(-1 * SEEK_SECONDS)}>
			<Icons name={Icon.Backward} class="text-primary-300" />
		</button>
		<button
			on:click={() => toggle(paused)}
			class="w-12 h-12 rounded-full flex justify-center items-center bg-gradient-to-br from-primary-300 to-primary-500"
		>
			{#if $playing}
				<Icons name={Icon.Pause} />
			{:else}
				<Icons name={Icon.Play} />
			{/if}
		</button>

		<button on:click={() => seekBy(SEEK_SECONDS)}>
			<Icons name={Icon.Forward} class="text-primary-300" />
		</button>

		<button
			class="w-6 h-4 flex items-center justify-center rounded-full bg-primary-300"
			on:click={handlePlaybackSpeedClick}
		>
			<span class="text-[8px] font-semibold text-white">{$playbackRate}x</span>
		</button>
	</div>

	<div class="mt-4 w-full flex items-center space-x-2">
		<span class="text-sm text-primary-800">{toHHMMSS($currentTime)}</span>
		<PlayerSlider max={$duration} bind:value={$currentTime} />
		<span class="text-sm text-primary-800">{toHHMMSS($duration)}</span>
		<VolumeControl />
	</div>
</div>

<style lang="scss">
	:global(.icon-4) {
		@apply w-4 h-4;
	}
</style>
