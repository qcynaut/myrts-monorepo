<script lang="ts">
	import Card from '$lib/components/card/Card.svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import InputSearch from '$lib/components/input/InputSearch.svelte';
	import type { Avs } from '$lib/types/response';
	import { setLoading } from '$lib/stores/ui';
	import { getAvs } from '$lib/service/avs';
	import { authStore } from '$lib/stores/auth';
	import { notify } from '$lib/stores/toast';
	import { onMount } from 'svelte';
	import CheckBox from '$lib/components/input/CheckBox.svelte';
	import Server from '$lib/components/icon/Server.svelte';
	import BadgeLabel from '$lib/components/badge/BadgeLabel.svelte';
	import Microphone from '$lib/components/icon/Microphone.svelte';
	import Modal from '$lib/components/modal/Modal.svelte';
	import Spinner from '$lib/components/animation/Spinner.svelte';
	import MyRTS from '$lib/rtc';
	import { STREAM_URL } from '$lib/const/urls';
	import { Wave } from '@foobar404/wave';
	import { formatSecondToDuration } from '$lib/utils/format';
	import Square from '$lib/components/icon/Square.svelte';
	import Play from '$lib/components/icon/Play.svelte';
	import Pause from '$lib/components/icon/Pause.svelte';
	import VolumeUp from '$lib/components/icon/VolumeUp.svelte';
	import Slider from '$lib/components/input/Slider.svelte';

	let avs: Avs[] = [];
	let selectedAvs: Avs[] = [];
	let streaming: MyRTS | null = null;
	let pending: boolean = true;
	let paused: boolean = false;
	let timeCount: number = 0;
	let counter: NodeJS.Timeout | null = null;
	let audioEl: HTMLAudioElement;
	let waveEl: HTMLCanvasElement;
	let wave: Wave | null = null;
	let volume = 100;

	const loadAvs = async () => {
		const token = $authStore.token ?? '';
		setLoading(true);
		const res = await getAvs(token);
		setLoading(false);
		if (res.error || res.result == null) {
			notify('Gagal mengambil data avs', 'error');
			return;
		}
		avs = res.result;
	};

	const select = (av: Avs) => {
		if (selectedAvs.includes(av)) {
			selectedAvs = selectedAvs.filter((a) => a.id != av.id);
		} else {
			selectedAvs = [...selectedAvs, av];
		}
	};

	const selectAll = () => {
		if (selectedAvs.length == avs.length) {
			selectedAvs = [];
		} else {
			selectedAvs = avs;
		}
	};

	const startStreaming = async () => {
		if (streaming) return;
		pending = true;
		volume = 100;
		const token = $authStore.token ?? '';
		if (selectedAvs.length == 0) {
			notify('Tidak ada avs yang dipilih', 'error');
			return;
		}
		try {
			streaming = await MyRTS.create(STREAM_URL);
			streaming.connected_handler = () => {
				pending = false;
				let m = streaming?.getMediaStream();
				wave = new Wave(audioEl, waveEl, true);
				wave.addAnimation(
					new wave.animations.Shine({
						lineColor: '#4C4EBF'
					})
				);
				if (m) {
					audioEl.srcObject = m;
					audioEl.volume = 0;
					audioEl.play();
				}

				counter = setInterval(() => {
					if (!paused) timeCount++;
				}, 1000);
			};
			streaming.error_handler = (e) => {
				pending = false;
				notify(e.toString(), 'error');
				setTimeout(() => {
					streaming = null;
				}, 3000);
			};
			await streaming.start(
				token,
				selectedAvs.map((av) => av.unique_id)
			);
		} catch (error) {
			notify('Gagal memulai streaming', 'error');
		}
	};

	const play = () => {
		if (!streaming) return;
		if (paused) {
			streaming.resume();
			paused = false;
		}
	};

	const pause = () => {
		if (!streaming) return;
		if (!paused) {
			streaming.pause();
			paused = true;
		}
	};

	const stop = () => {
		if (!streaming) return;
		streaming.stop();
		streaming = null;
		pending = true;
		paused = false;
		timeCount = 0;
		volume = 100;
		if (counter) {
			clearInterval(counter);
			counter = null;
		}
	};

	let search = '';

	onMount(() => {
		loadAvs();
	});

	$: {
		if (streaming) {
			const vol = volume / 100;
			streaming.volume(vol.toFixed(1));
		}
	}

	$: fail = selectedAvs.filter((av) => av.status != 1);
	$: filtered =
		search == ''
			? avs
			: avs.filter(
					(av) =>
						av.address?.includes(search) ||
						av.unique_id.includes(search) ||
						av.description?.includes(search)
			  );
</script>

<Modal open={streaming != null} class="w-10/12 md:w-2/3 h-[90%] md:h-2/3" closable={false}>
	<div class="p-6 flex flex-col md:flex-row gap-2 h-full">
		<Card class="p-4 bg-gray-300 w-full md:w-1/3 h-[40%] md:h-full flex flex-col">
			<h1 class="text-xs md:text-lg">
				Streaming ini <span class="font-semibold">tidak dapat dilakukan</span> di AVS berikut:
			</h1>
			<ul class="mt-4 flex-1 h-full overflow-scroll">
				{#each fail as av, i}
					<li class="flex flex-wrap items-center gap-2">
						{i + 1}. <Server class="w-4 h-4 fill-primary-600" /><span
							>{av.address ?? av.unique_id}</span
						>
					</li>
				{/each}
			</ul>
		</Card>
		<div class="w-full md:w-2/3 h-full p-4 flex justify-center items-center">
			<div class="flex items-center flex-col gap-2 {pending ? '' : 'hidden'}">
				<Spinner class="w-16 h-16 fill-primary-600" />
				<span>Menghubungkan</span>
				<audio bind:this={audioEl} class="hidden" />
				<canvas bind:this={waveEl} />
			</div>
			<div class="flex items-center flex-col gap-2 {pending ? 'hidden' : ''}">
				<audio bind:this={audioEl} />
				<canvas bind:this={waveEl} />
				<div class="mt-2">
					{formatSecondToDuration(timeCount)}
				</div>
				<div class="mt-2 flex items-center gap-2 w-full">
					<VolumeUp class="w-4 h-4 fill-primary-600" />
					<Slider min={10} max={500} bind:value={volume} />
				</div>
				<div class="mt-2 flex gap-2 items-center">
					{#if paused}
						<button class="p-2 rounded-full bg-green-600" on:click={play}>
							<Play class="w-6 h-6 fill-white" />
						</button>
					{:else}
						<button class="p-2 rounded-full bg-yellow-600" on:click={pause}>
							<Pause class="w-6 h-6 fill-white" />
						</button>
					{/if}
					<button class="p-2 rounded-full bg-red-600" on:click={stop}>
						<Square class="w-6 h-6 fill-white" />
					</button>
				</div>
			</div>
		</div>
	</div>
</Modal>

<DashboardInner title="Streaming" subtitle="Mulai Streaming">
	<div class="h-full w-full flex flex-col md:flex-row gap-4 py-4 px-4">
		<Card class="w-full md:w-1/4 bg-white p-4" rounded="xl" shadow="xl">
			<InputSearch rounded="full" placeholder="Cari avs" bind:value={search} />
			<div class="mt-4">
				<CheckBox
					checked={selectedAvs.length == avs.length}
					on:check={() => selectAll()}
					class="mb-2"
				>
					Pilih Semua
				</CheckBox>
				{#each filtered as av}
					<CheckBox
						checked={selectedAvs.includes(av)}
						on:check={() => select(av)}
						class="mb-2 pl-4"
					>
						<div
							class="flex items-center gap-2 {av.status != 1 ? 'fill-red-600' : 'fill-primary-600'}"
						>
							<Server class="w-4 h-4" />
							<span>{av.address ?? av.unique_id}</span>
						</div>
					</CheckBox>
				{/each}
			</div>
		</Card>
		<div class="left-container w-full h-full flex flex-col gap-4">
			{#if selectedAvs.length > 0}
				<Card class="p-4 bg-white hidden md:block" rounded="xl" shadow="xl">
					<h1 class="text-xl font-semibold">Dipilih</h1>
					<div class="mt-4 w-full flex gap-2 flex-wrap">
						{#each selectedAvs as av}
							<BadgeLabel size="md" name={av.address ?? av.unique_id} on:close={() => select(av)} />
						{/each}
					</div>
				</Card>
			{/if}
			<Card class="p-4 bg-white" rounded="xl" shadow="xl">
				<div class="p-4 w-full h-full flex justify-center items-center">
					<button
						class="p-4 md:p-8 flex items-center justify-center bg-green-600 rounded-full"
						on:click={startStreaming}
					>
						<Microphone class="w-8 h-8 md:w-12 md:h-12 fill-white" />
					</button>
				</div>
			</Card>
		</div>
	</div>
</DashboardInner>

<style lang="scss">
	.left-container {
		:global(:not(:first-child)) {
			@apply h-full md:h-1/3;
		}
	}
</style>
