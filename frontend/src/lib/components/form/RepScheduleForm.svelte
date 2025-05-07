<script lang="ts">
	import { Button, Label, Select } from 'flowbite-svelte';
	import ModalSelect from './ModalSelect.svelte';
	import DatePicker from './DatePicker.svelte';
	import TimePicker from './TimePicker.svelte';
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import { getAvs } from '$lib/service/avs';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import { Icon } from '$lib/const/icon';
	import { Range } from 'flowbite-svelte';
	import Icons from '../ui/Icons.svelte';
	import { writable, type Writable } from 'svelte/store';
	import { getRecords } from '$lib/service/record';
	import AudioPlayer from '../ui/AudioPlayer.svelte';
	import MiniPlayer from '../ui/MiniPlayer.svelte';
	import { createSchedule } from '$lib/service/schedule';
	import { goto } from '$app/navigation';
	import { DASHBOARD_SCHEDULE } from '$lib/const/navigation';
	import FormInput from './FormInput.svelte';

	let avs: { value: number; name: string }[] = [];
	let records: { value: number; name: string }[] = [];
	let records_urls: { value: number; name: string }[] = [];
	let record_url = '';
	let name = '';

	let repetitions = [
		{ value: 1, name: 'Bulanan' },
		{ value: 2, name: 'Mingguan' }
	];
	let repetition: number | null = null;
	let weeks = [
		{ value: 1, name: 'Minggu ke 1' },
		{ value: 2, name: 'Minggu ke 2' },
		{ value: 3, name: 'Minggu ke 3' },
		{ value: 4, name: 'Minggu ke 4' },
		{ value: 5, name: 'Minggu ke 5' }
	];
	let selectedWeeks: number[] = [];
	let selectedDates: number[] = [];
	let days = [
		{ value: 1, name: 'Senin' },
		{ value: 2, name: 'Selasa' },
		{ value: 3, name: 'Rabu' },
		{ value: 4, name: 'Kamis' },
		{ value: 5, name: 'Jumat' },
		{ value: 6, name: 'Sabtu' },
		{ value: 7, name: 'Minggu' }
	];
	let selectedDays: number[] = [];
	let selectedTimes: string[] = [];
	let selectedAvs: number[] = [];
	let selectedRecord: number | null = null;
	const selectedAvsVolume: Writable<{ [key: number]: number }> = writable({});

	const load = async () => {
		const token = $authStore.token ?? '';
		setLoading(true);
		const res = await getAvs(token);
		const res1 = await getRecords(token);
		setLoading(false);
		if (res.error || !res.result) return toastError('Tidak dapat mengambil data avs');
		avs = res.result
			.filter((a) => a.status != 3)
			.map((a) => ({ value: a.id, name: a.address ?? a.unique_id }));
		if (res1.error || !res1.result) return toastError('Tidak dapat mengambil data record');
		records = res1.result
			.filter((r) => r.status == 1 && r.user_id == $authStore.user?.id)
			.map((r) => ({ value: r.id, name: r.name }));
		records_urls = res1.result
			.filter((r) => r.status == 1 && r.user_id == $authStore.user?.id)
			.map((r) => ({ value: r.id, name: r.file_url }));
	};

	const onVolumeChange = (e: Event, av: number) => {
		const value = (e.target as HTMLInputElement).value;
		$selectedAvsVolume[av] = Number(value);
	};

	const submit = async () => {
		const token = $authStore.token ?? '';
		if (name == '') return toastError('Masukkan nama');
		if (!repetition) return toastError('Pilih tipe repetisi');
		if (selectedAvs.length == 0) return toastError('Pilih AVS');
		if (selectedRecord == null) return toastError('Pilih rekaman');
		if (selectedTimes.length == 0) return toastError('Pilih jam');
		let volumes = [];
		for (const av of selectedAvs) {
			let vol = $selectedAvsVolume[av] ?? 100;
			// since in the display was 10% - 500%, and the actual we need is between 0.1 - 5.0 which mean 1.0 == 100, then we need to divide by 10 and make them to be double
			let voldoble: number = vol / 100;
			volumes.push(`${av}:${voldoble.toFixed(1)}`);
		}
		if (repetition == 1) {
			if (selectedDates.length == 0) return toastError('Pilih tanggal');
			setLoading(true);
			const res = await createSchedule(
				token,
				selectedDates,
				[],
				selectedAvs,
				1,
				null,
				name,
				selectedRecord,
				selectedTimes,
				volumes,
				[],
				null
			);
			setLoading(false);
			if (res.error) return toastError('Tidak dapat membuat jadwal');
			toastSuccess('Jadwal berhasil dibuat');
			goto(DASHBOARD_SCHEDULE, { replaceState: true });
		} else {
			if (selectedDays.length == 0) return toastError('Pilih hari');
			if (selectedWeeks.length == 0) return toastError('Pilih minggu');
			setLoading(true);
			const res = await createSchedule(
				token,
				[],
				selectedDays,
				selectedAvs,
				1,
				null,
				name,
				selectedRecord,
				selectedTimes,
				volumes,
				selectedWeeks,
				null
			);
			setLoading(false);
			if (res.error) return toastError('Tidak dapat membuat jadwal');
			toastSuccess('Jadwal berhasil dibuat');
			goto(DASHBOARD_SCHEDULE, { replaceState: true });
		}
	};

	onMount(() => {
		load();
	});

	$: {
		record_url = records_urls.find((r) => r.value == selectedRecord)?.name ?? '';
	}
</script>

<FormInput bind:value={name} label="Nama Jadwal" />
<div class="mb-6">
	<Label>
		Tipe repetisi
		<Select class="mt-2" bind:value={repetition} items={repetitions} placeholder="Pilih Tipe" />
	</Label>
</div>
{#if repetition == 1}
	<DatePicker onlyDate={true} bind:value={selectedDates} label="Setiap Tanggal" />
	<TimePicker label="Setiap Jam" bind:value={selectedTimes} />
{:else if repetition == 2}
	<ModalSelect label="Pilih Minggu" items={weeks} bind:value={selectedWeeks} />
	<ModalSelect label="Pilih Hari" items={days} bind:value={selectedDays} />
	<TimePicker label="Setiap Jam" bind:value={selectedTimes} />
{/if}
<ModalSelect label="Pilih AVS" items={avs} bind:value={selectedAvs} icon={Icon.Server} />
{#if selectedAvs.length > 0}
	<div class="mb-6">
		<Label class="mt-2">Volume AVS</Label>
		{#each selectedAvs as av}
			<div class="mt-2 flex flex-col gap-2">
				<span class="flex gap-2 items-center">
					<Icons name={Icon.Server} />
					{avs.find((a) => a.value == av)?.name}:
				</span>
				<div class="flex gap-2 items-center">
					<Icons name={Icon.VolumeUp} />
					<Range
						value={$selectedAvsVolume[av]}
						min={10}
						max={500}
						on:change={(e) => onVolumeChange(e, av)}
					/>
					{#if $selectedAvsVolume[av]}
						{$selectedAvsVolume[av]}
					{:else}
						100
					{/if}
				</div>
			</div>
		{/each}
	</div>
{/if}
<div
	class="mb-6 {selectedRecord
		? 'flex flex-col md:flex-row gap-2 items-center justify-between'
		: ''}"
>
	<div class={selectedRecord ? 'md:w-1/2' : ''}>
		<Label>
			Pilih rekaman
			<Select class="mt-2" bind:value={selectedRecord} items={records} placeholder="Pilih Tipe" />
		</Label>
	</div>
	{#if selectedRecord}
		<AudioPlayer bind:src={record_url}>
			<MiniPlayer />
		</AudioPlayer>
	{/if}
</div>
<div class="mb-8 flex justify-center">
	<Button on:click={submit}>Submit</Button>
</div>
