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
	let years: { value: number; name: string }[] = [
		{ value: 2023, name: '2023' },
		{ value: 2024, name: '2024' },
		{ value: 2025, name: '2025' },
		{ value: 2026, name: '2026' }
	];
	let year: number = 2023;
	let months: { value: number; name: string }[] = [
		{ value: 1, name: 'Januari' },
		{ value: 2, name: 'Februari' },
		{ value: 3, name: 'Maret' },
		{ value: 4, name: 'April' },
		{ value: 5, name: 'Mei' },
		{ value: 6, name: 'Juni' },
		{ value: 7, name: 'Juli' },
		{ value: 8, name: 'Agustus' },
		{ value: 9, name: 'September' },
		{ value: 10, name: 'Oktober' },
		{ value: 11, name: 'November' },
		{ value: 12, name: 'Desember' }
	];
	let month = 1;
	let record_url = '';
	let name = '';
	let selectedDates: number[] = [];
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
		if (selectedAvs.length == 0) return toastError('Pilih AVS terlebih dahulu');
		if (selectedRecord == null) return toastError('Pilih rekaman');
		if (selectedDates.length == 0) return toastError('Pilih tanggal');
		if (selectedTimes.length == 0) return toastError('Pilih jam');
		if (name == '') return toastError('Masukkan nama');
		let volumes = [];
		for (const av of selectedAvs) {
			let vol = $selectedAvsVolume[av] ?? 100;
			// since in the display was 10% - 500%, and the actual we need is between 0.1 - 5.0 which mean 1.0 == 100, then we need to divide by 10 and make them to be double
			let voldoble: number = vol / 100;
			volumes.push(`${av}:${voldoble.toFixed(1)}`);
		}
		setLoading(true);
		const res = await createSchedule(
			token,
			selectedDates,
			[],
			selectedAvs,
			2,
			month,
			name,
			selectedRecord,
			selectedTimes,
			volumes,
			[],
			year
		);
		setLoading(false);
		if (res.error) return toastError('Tidak dapat membuat jadwal');
		toastSuccess('Berhasil membuat jadwal');
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
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
		Tahun
		<Select
			class="mt-2"
			bind:value={year}
			items={years}
			placeholder="Pilih Tahun"
			on:change={(e) => (month = month)}
		/>
	</Label>
</div>
<div class="mb-6">
	<Label>
		Bulan
		<Select class="mt-2" bind:value={month} items={months} placeholder="Pilih Bulan" />
	</Label>
</div>
<!-- <DatePicker {year} {month} onlyDate={true} bind:value={selectedDates} label="Pilih Tanggal" /> -->
<TimePicker label="Pilih Jam" bind:value={selectedTimes} />
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
