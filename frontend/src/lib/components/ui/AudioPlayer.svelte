<script lang="ts">
	import { setAudioContext } from '$lib/context/audio';
	import { onMount, afterUpdate } from 'svelte';
	import { derived, writable } from 'svelte/store';

	export let src: string;

	let prevSrc: string | undefined = undefined;

	let duration = writable(0);
	let currentTime = writable(0);
	let volume = writable(1);
	let muted = writable(false);
	let ended = writable(false);
	let repeat = writable(false);
	let playbackRate = writable(1);
	let paused = writable(true);
	let playing = derived(paused, ($paused) => !$paused);

	let audio: HTMLAudioElement | undefined;

	/**
	 * Reactives
	 */

	$: {
		if (prevSrc !== src) {
			// fix $paused store not sync with audio.paused on src props change
			setTimeout(() => {
				if ($paused) {
					audio?.pause();
				} else {
					audio?.play();
				}
			}, 0);
		}

		prevSrc = src;
	}
	/**
	 * Mounted
	 */
	onMount(() => {
		if (audio?.duration) {
			$duration = audio?.duration;
		}
	});

	/**
	 * Methods
	 */

	const seekBy = (n: number) => {
		$currentTime += n;
	};

	const seekTo = (t: number) => {
		$currentTime = t;
	};

	/**
	 * Context
	 */

	setAudioContext({
		currentTime,
		repeat,
		duration,
		playing,
		volume,
		muted,
		ended,
		paused,
		playbackRate,
		seekBy,
		seekTo
	});
</script>

<div>
	<audio
		bind:volume={$volume}
		bind:duration={$duration}
		bind:currentTime={$currentTime}
		bind:muted={$muted}
		bind:paused={$paused}
		bind:ended={$ended}
		bind:playbackRate={$playbackRate}
		loop={$repeat}
		bind:this={audio}
		{src}
		style="display: none;"
	/>

	<slot />
</div>
