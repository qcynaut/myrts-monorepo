<script lang="ts">
	import { Icon } from '$lib/const/icon';
	import { toastError } from '$lib/utils/toast';
	import Icons from './Icons.svelte';

	export let cancel = () => {};
	export let save = (file: File) => {};

	let recorder: MediaRecorder | null = null;
	let chunks: Blob[] = [];
	let record_seconds = 0;
	let paused = false;
	let done = false;

	const start = async () => {
		try {
			const device = await navigator.mediaDevices.getUserMedia({ audio: true });
			recorder = new MediaRecorder(device);
			recorder.ondataavailable = (e) => {
				chunks.push(e.data);
			};
			recorder.start();
			record_seconds = 0;
			chunks = [];
			while (recorder) {
				if (!paused) {
					record_seconds += 1;
				}
				await new Promise((resolve) => setTimeout(resolve, 1000));
			}
		} catch (error) {
			toastError('Tidak dapat mengakses mikrofon');
		}
	};

	const playPause = () => {
		if (paused) {
			recorder!.resume();
		} else {
			recorder!.pause();
		}
		paused = !paused;
	};

	const stop = () => {
		if (recorder) {
			recorder.stop();
			recorder.stream.getTracks().forEach((track) => track.stop());
		}
		done = true;
		recorder = null;
	};

	const formatTime = (seconds: number) => {
		// format seconds to hh:mm:ss
		const date = new Date(seconds * 1000);
		const hh = date.getUTCHours().toString().padStart(2, '0');
		const mm = date.getUTCMinutes().toString().padStart(2, '0');
		const ss = date.getUTCSeconds().toString().padStart(2, '0');
		return `${hh}:${mm}:${ss}`;
	};

	const saveHandler = () => {
		if (chunks.length > 0) {
			const blob = new Blob(chunks, { type: 'audio/mpeg' });
			// create random name based on timestamp dd-mm-yyyy-hh-mm-ss
			const date = new Date();
			const name = `${date.getDate()}-${date.getMonth()}-${date.getFullYear()}-${date.getHours()}-${date.getMinutes()}-${date.getSeconds()}`;
			const file = new File([blob], `${name}.mp3`);
			save(file);
		}
		cancel();
	};
</script>

<div class="p-4 bg-white rounded-xl w-2/3 h-1/2 overflow-x-auto md:w-1/3 md:h-1/2">
	<div class="flex justify-end">
		<button
			on:click={() => {
				if (!done) {
					stop();
				}
				cancel();
			}}><Icons name={Icon.Times} /></button
		>
	</div>
	<div class="flex py-8 justify-center gap-6">
		{#if !recorder}
			<button
				on:click={start}
				class="w-16 h-16 flex justify-center items-center text-4xl text-white bg-primary-500 rounded-full"
				><Icons name={Icon.Microphone} /></button
			>
		{:else}
			<button
				on:click={playPause}
				class="w-16 h-16 flex justify-center items-center text-4xl text-white {paused
					? 'bg-primary-500'
					: 'bg-yellow-500'} rounded-full"><Icons name={paused ? Icon.Play : Icon.Pause} /></button
			>
			<button
				on:click={stop}
				class="w-16 h-16 flex justify-center items-center text-4xl bg-red-500 text-white stop rounded-full"
				><Icons name={Icon.Stop} /></button
			>
		{/if}
	</div>
	{#if recorder}
		<div class="text-center text-xl">
			{formatTime(record_seconds)}
		</div>
	{/if}
	{#if done}
		<div class="my-8 flex justify-center">
			<button
				on:click={saveHandler}
				class="px-4 py-2 flex gap-2 items-center bg-primary-500 rounded-l-full rounded-r-full text-white"
				>Simpan</button
			>
		</div>
	{/if}
</div>
