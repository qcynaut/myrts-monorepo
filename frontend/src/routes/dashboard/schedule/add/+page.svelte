<script lang="ts">
	import type { BreadCrumbItem } from '$lib/components/breadcrumb/types';
	import CheckBox from '$lib/components/input/CheckBox.svelte';
	import Input from '$lib/components/input/Input.svelte';
	import InputRadio from '$lib/components/input/InputRadio.svelte';
	import Label from '$lib/components/input/Label.svelte';
	import Select from '$lib/components/input/Select.svelte';
	import Clock from '$lib/components/icon/Clock.svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import { DASHBOARD_SCHEDULE, DASHBOARD_SCHEDULE_ADD } from '$lib/const/navigation';
	import type { NameValue } from '$lib/types/components';
	import Modal from '$lib/components/modal/Modal.svelte';
	import Times from '$lib/components/icon/Times.svelte';
	import { TimePicker } from 'svelte-time-picker';
	import '../../../../styles/timepicker.scss';
	import BadgeLabel from '$lib/components/badge/BadgeLabel.svelte';
	import Plus from '$lib/components/icon/Plus.svelte';
	import SelectModal from '$lib/components/input/SelectModal.svelte';
	import Button from '$lib/components/button/Button.svelte';
	import MultiDateOnlyPicker from '$lib/components/input/MultiDateOnlyPicker.svelte';
	import { setLoading } from '$lib/stores/ui';
	import { authStore } from '$lib/stores/auth';
	import { getAvs } from '$lib/service/avs';
	import { getRecords } from '$lib/service/record';
	import { notify } from '$lib/stores/toast';
	import { onMount } from 'svelte';
	import Slider from '$lib/components/input/Slider.svelte';
	import Server from '$lib/components/icon/Server.svelte';
	import VolumeUp from '$lib/components/icon/VolumeUp.svelte';
	import { createSchedule } from '$lib/service/schedule';
	import { goto } from '$app/navigation';

	let scheduleName: string = '';

	const breadcrumbs: BreadCrumbItem[] = [
		{
			name: 'Daftar Jadwal',
			url: DASHBOARD_SCHEDULE
		},
		{
			name: 'Tambah Jadwal',
			url: DASHBOARD_SCHEDULE_ADD
		}
	];

	const repetitionOptions: NameValue<number>[] = [
		{
			name: 'Ya',
			value: 1
		},
		{
			name: 'Tidak',
			value: 2
		}
	];
	let selectedRepetition: number | undefined = undefined;

	const repetitionType: NameValue<number>[] = [
		{
			name: 'Bulanan',
			value: 1
		},
		{
			name: 'Mingguan',
			value: 2
		}
	];
	let selectedRepetitionType: number | undefined = undefined;

	const weeks = [1, 2, 3, 4, 5];
	let selectedWeek: number[] = [];

	const selectWeek = (week: number) => {
		if (selectedWeek.includes(week)) {
			selectedWeek = selectedWeek.filter((w) => w != week);
		} else {
			selectedWeek = [...selectedWeek, week];
		}
	};

	const selectAllWeek = () => {
		if (selectedWeek.length == weeks.length) {
			selectedWeek = [];
		} else {
			selectedWeek = weeks;
		}
	};

	const days: NameValue<number>[] = [
		{
			name: 'Minggu',
			value: 1
		},
		{
			name: 'Senin',
			value: 2
		},
		{
			name: 'Selasa',
			value: 3
		},
		{
			name: 'Rabu',
			value: 4
		},
		{
			name: 'Kamis',
			value: 5
		},
		{
			name: 'Jumat',
			value: 6
		},
		{
			name: 'Sabtu',
			value: 7
		}
	];
	let selectedDay: number[] = [];

	const selectDay = (day: number) => {
		if (selectedDay.includes(day)) {
			selectedDay = selectedDay.filter((d) => d != d);
		} else {
			selectedDay = [...selectedDay, day];
		}
	};

	const selectAllDay = () => {
		if (selectedDay.length == days.length) {
			selectedDay = [];
		} else {
			selectedDay = days.map((d) => d.value);
		}
	};

	let timeModal = false;
	let times: string[] = [];

	const addTime = (e: Event) => {
		const detail = (e as Event & { detail: Date }).detail;
		const time = [
			detail.getHours().toString().padStart(2, '0'),
			detail.getMinutes().toString().padStart(2, '0')
		].join(':');
		if (!times.includes(time)) {
			times = [...times, time];
		}
		timeModal = false;
	};
	const removeTime = (time: string) => {
		times = times.filter((t) => t != time);
	};

	let avs: NameValue<number>[] = [];
	let selectedAvs: NameValue<number>[] = [];

	const genYears = () => {
		const years = [];
		const date = new Date();
		for (let i = 0; i < 5; i++) {
			years.push({
				name: `${date.getFullYear() + i}`,
				value: date.getFullYear() + i
			});
		}
		return years;
	};
	let selectedYear: number | undefined = undefined;

	const months: NameValue<number>[] = [
		{
			name: 'Januari',
			value: 1
		},
		{
			name: 'Februari',
			value: 2
		},
		{
			name: 'Maret',
			value: 3
		},
		{
			name: 'April',
			value: 4
		},
		{
			name: 'Mei',
			value: 5
		},
		{
			name: 'Juni',
			value: 6
		},
		{
			name: 'Juli',
			value: 7
		},
		{
			name: 'Agustus',
			value: 8
		},
		{
			name: 'September',
			value: 9
		},
		{
			name: 'Oktober',
			value: 10
		},
		{
			name: 'November',
			value: 11
		},
		{
			name: 'Desember',
			value: 12
		}
	];
	let selectedMonth: number | undefined = undefined;
	let selectedDates: number[] = [];

	let records: NameValue<number>[] = [];
	let selectedRecord: number | undefined = undefined;
	let volumes: Record<number, number> = {};

	const load = async () => {
		const token = $authStore.token || '';
		setLoading(true);
		const avsRes = await getAvs(token);
		const recordsRes = await getRecords(token);
		setLoading(false);
		if (avsRes.error) {
			notify(avsRes.error.error, 'error');
		}
		if (recordsRes.error) {
			notify(recordsRes.error.error, 'error');
		}
		if (avsRes.result) {
			avs = avsRes.result.map((a) => ({
				name: a.address || a.unique_id,
				value: a.id
			}));
		}
		if (recordsRes.result) {
			records = recordsRes.result.map((r) => ({
				name: r.name,
				value: r.id
			}));
		}
	};

	const getVolumes = () => {
		const res = [];
		const added = [];
		for (const vol in volumes) {
			const volume = volumes[vol];
			res.push(`${vol}: ${volume}`);
			added.push(Number(vol));
		}
		for (const av of selectedAvs) {
			if (!added.includes(av.value)) {
				res.push(`${av.name}:1.0`);
			}
		}
		return res;
	};

	const submitMonthly = async () => {
		const token = $authStore.token || '';
		setLoading(true);
		const res = await createSchedule(
			token,
			selectedDates,
			[],
			selectedAvs.map((av) => av.value),
			1,
			null,
			scheduleName,
			selectedRecord!,
			times,
			getVolumes(),
			[],
			null
		);
		setLoading(false);
		if (res.error || !res.result) {
			if (res.error) {
				notify(res.error.error, 'error');
			} else {
				notify('Gagal membuat jadwal', 'error');
			}
			return;
		}
		notify('Jadwal berhasil dibuat', 'success');
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
	};

	const submitWeekly = async () => {
		const token = $authStore.token || '';
		setLoading(true);
		const res = await createSchedule(
			token,
			[],
			selectedDay,
			selectedAvs.map((av) => av.value),
			1,
			null,
			scheduleName,
			selectedRecord!,
			times,
			getVolumes(),
			selectedWeek,
			null
		);
		setLoading(false);
		if (res.error || !res.result) {
			if (res.error) {
				notify(res.error.error, 'error');
			} else {
				notify('Gagal membuat jadwal', 'error');
			}
			return;
		}
		notify('Jadwal berhasil dibuat', 'success');
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
	};

	const submitNonRepetitive = async () => {
		const token = $authStore.token || '';
		setLoading(true);
		const res = await createSchedule(
			token,
			selectedDates,
			[],
			selectedAvs.map((av) => av.value),
			2,
			selectedMonth!,
			scheduleName,
			selectedRecord!,
			times,
			getVolumes(),
			[],
			selectedYear!
		);
		setLoading(false);
		if (res.error || !res.result) {
			if (res.error) {
				notify(res.error.error, 'error');
			} else {
				notify('Gagal membuat jadwal', 'error');
			}
			return;
		}
		notify('Jadwal berhasil dibuat', 'success');
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
	};

	const submit = async () => {
		if (scheduleName == '') return notify('Masukkan judul jadwal', 'error');
		if (!selectedRepetition) return notify('Pilih tipe jadwal', 'error');
		if (selectedAvs.length == 0) return notify('Pilih avs', 'error');
		if (!selectedRecord) return notify('Pilih rekaman', 'error');
		if (selectedRepetition == 1) {
			if (!selectedRepetitionType) return notify('Pilih tipe repetisi', 'error');
			if (selectedRepetitionType == 1) {
				if (selectedDates.length == 0) return notify('Pilih tanggal', 'error');
				await submitMonthly();
			} else {
				if (selectedWeek.length == 0) return notify('Pilih minggu', 'error');
				if (selectedDay.length == 0) return notify('Pilih hari', 'error');
				await submitWeekly();
			}
		} else {
			if (!selectedYear) return notify('Pilih tahun', 'error');
			if (!selectedMonth) return notify('Pilih bulan', 'error');
			if (selectedDates.length == 0) return notify('Pilih tanggal', 'error');
			await submitNonRepetitive();
		}
	};

	onMount(() => {
		load();
	});
</script>

<DashboardInner title="Tambah Jadwal" {breadcrumbs}>
	<div class="w-full flex-1 bg-white p-4 overflow-hidden">
		<div class="p-6 border border-gray-300 rounded-lg h-full overflow-y-auto">
			<h1 class="mb-8 text-xl font-semibold text-center">Data Jadwal</h1>
			<Label>
				Judul
				<Input type="text" placeholder="Masukan Judul" bind:value={scheduleName} />
			</Label>
			<div class="mb-4">
				Repetisi
				<div class="pl-4">
					<InputRadio items={repetitionOptions} bind:selected={selectedRepetition} />
				</div>
			</div>
			{#if selectedRepetition}
				<div class="mx-2 p-4 rounded-md bg-gray-100 mb-4">
					{#if selectedRepetition === 1}
						<h1 class="mb-4 font-semibold">Jadwal Repetisi</h1>
						<Select
							items={repetitionType}
							bind:selected={selectedRepetitionType}
							label="Tipe Repetisi"
							placeholder="Pilih Tipe"
						/>
						{#if selectedRepetitionType}
							{#if selectedRepetitionType === 1}
								<MultiDateOnlyPicker
									year={selectedYear}
									month={selectedMonth}
									bind:selected={selectedDates}
									noheader
								/>
							{:else}
								<Label>Minggu</Label>
								<div class="mt-4 flex gap-8 flex-wrap">
									<Label class="flex items-center gap-2">
										<CheckBox
											checked={selectedWeek.length == weeks.length}
											on:check={() => selectAllWeek()}
										/>
										Pilih Semua
									</Label>
									{#each weeks as week}
										<Label class="flex items-center gap-2">
											<CheckBox
												checked={selectedWeek.includes(week)}
												on:check={() => selectWeek(week)}
											/>
											Minggu ke-{week}
										</Label>
									{/each}
								</div>
								<hr class="mb-2" />
								<Label>Hari</Label>
								<div class="mt-4 flex gap-8 flex-wrap">
									<Label class="flex items-center gap-2">
										<CheckBox
											checked={selectedDay.length == days.length}
											on:check={() => selectAllDay()}
										/>
										Pilih Semua
									</Label>
									{#each days as day}
										<Label class="flex items-center gap-2">
											<CheckBox
												checked={selectedDay.includes(day.value)}
												on:check={() => selectDay(day.value)}
											/>
											{day.name}
										</Label>
									{/each}
								</div>
							{/if}
						{/if}
					{:else}
						<h1 class="mb-4 font-semibold">Jadwal Non-Repetisi</h1>
						<Select
							items={genYears()}
							bind:selected={selectedYear}
							placeholder="Pilih tahun"
							label="Tahun"
						/>
						<Select
							items={months}
							bind:selected={selectedMonth}
							placeholder="Pilih bulan"
							label="Bulan"
						/>
						{#if selectedYear && selectedMonth}
							<MultiDateOnlyPicker
								year={selectedYear}
								month={selectedMonth}
								bind:selected={selectedDates}
							/>
						{/if}
					{/if}
					<hr class="mb-2" />
					<Label>
						Plih Waktu
						<button
							on:click={() => (timeModal = true)}
							class="flex justify-between items-center w-full text-start p-2 rounded bg-white border-gray-200 border"
						>
							<span>Pilih</span>
							<Clock class="w-6 h-6 fill-gray-500" />
						</button>
					</Label>
					{#if times.length > 0}
						<div class="flex gap-2 flex-wrap items-center">
							{#each times as time}
								<BadgeLabel
									name={time}
									bg="primary-400"
									rounded="md"
									on:close={() => removeTime(time)}
								/>
							{/each}
							<button
								class="border border-primary-400 rounded-md text-primary-400 flex gap-2 items-center p-2"
								on:click={() => (timeModal = true)}
							>
								<Plus class="w-4 h-4 fill-primary-400" />
								<span>Tambah</span>
							</button>
						</div>
					{/if}
				</div>
				<SelectModal
					placeholder="Dapat pilih lebih dari satu"
					title="AVS"
					searchText="Cari AVS"
					items={avs}
					bind:selected={selectedAvs}
				/>
				{#if selectedAvs.length > 0}
					<div class="mx-2 p-4 rounded-md bg-gray-100 mb-4">
						<h1 class="mb-4 font-semibold">Atur Volume</h1>
						{#each selectedAvs as av}
							<div class="flex flex-col gap-2 mb-">
								<div class="flex gap-2">
									<Server class="w-4 h-4 fill-primary-600" />
									<span>{av.name}</span>
								</div>
								<div class="flex gap-2">
									<VolumeUp class="w-4 h-4 fill-primary-600" />
									<Slider bind:value={volumes[av.value]} min={0.1} max={5.0} />
									<span>{((volumes[av.value] ?? 1.0) * 100).toString().split('.')[0]}%</span>
								</div>
							</div>
						{/each}
					</div>
				{/if}
				<Select
					label="Rekaman"
					placeholder="Pilih Rekaman"
					items={records}
					bind:selected={selectedRecord}
				/>
			{/if}
			<div class="flex justify-end">
				<Button class="bg-primary-600 text-white" on:click={submit}>Kirim</Button>
			</div>
		</div>
	</div>
</DashboardInner>

<Modal class="w-3/4 md:w-1/3 lg:w-1/4" bind:open={timeModal} closable={false}>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span>Pilih Waktu</span>
		<button class="p-2 rounded-full bg-white bg-opacity-25" on:click={() => (timeModal = false)}>
			<Times class="w-4 h-4 fill-white" />
		</button>
	</div>
	<div class="p-4">
		<TimePicker
			options={{
				clockClassName: 'clock',
				timeClassName: 'time',
				bgColor: '#636cce',
				hasButtons: true,
				buttonOk: 'Pilih',
				buttonClassName: 'btn',
				buttonBarClassName: 'btn-bar',
				is24h: true
			}}
			on:ok={addTime}
		/>
	</div>
</Modal>
