<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import BadgeLabel from '$lib/components/badge/BadgeLabel.svelte';
	import type { BreadCrumbItem } from '$lib/components/breadcrumb/types';
	import Button from '$lib/components/button/Button.svelte';
	import Clock from '$lib/components/icon/Clock.svelte';
	import Server from '$lib/components/icon/Server.svelte';
	import VolumeUp from '$lib/components/icon/VolumeUp.svelte';
	import Announcement from '$lib/components/images/Announcement.svelte';
	import CheckBox from '$lib/components/input/CheckBox.svelte';
	import Input from '$lib/components/input/Input.svelte';
	import InputRadio from '$lib/components/input/InputRadio.svelte';
	import Label from '$lib/components/input/Label.svelte';
	import MultiDateOnlyPicker from '$lib/components/input/MultiDateOnlyPicker.svelte';
	import Select from '$lib/components/input/Select.svelte';
	import SelectModal from '$lib/components/input/SelectModal.svelte';
	import Slider from '$lib/components/input/Slider.svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import { DASHBOARD_SCHEDULE } from '$lib/const/navigation';
	import { getAvs } from '$lib/service/avs';
	import { getRecords } from '$lib/service/record';
	import { getScheduleById, updateScheduleVolume } from '$lib/service/schedule';
	import { authStore } from '$lib/stores/auth';
	import { notify } from '$lib/stores/toast';
	import { setLoading } from '$lib/stores/ui';
	import type { NameValue } from '$lib/types/components';
	import type { Records, Schedule } from '$lib/types/response';
	import { onMount } from 'svelte';

	let schedule: Schedule | undefined = undefined;

	const breadcrumbs: BreadCrumbItem[] = [
		{
			name: 'Daftar Jadwal',
			url: DASHBOARD_SCHEDULE
		},
		{
			name: 'Edit Jadwal',
			url: DASHBOARD_SCHEDULE + '/edit'
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
	const weeks = [1, 2, 3, 4, 5];
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

	let avs: NameValue<number>[] = [];
	let volumes: Record<number, number> = {};
	let record: Records | null = null;

	const load = async () => {
		const id = $page.params.id;
		if (isNaN(Number(id))) {
			notify('Jadwal tidak ditemukan', 'error');
			setTimeout(() => {
				goto(DASHBOARD_SCHEDULE);
			}, 1000);
			return;
		}
		const token = $authStore.token || '';
		setLoading(true);
		const res = await getScheduleById(token, Number(id));
		const res2 = await getAvs(token);
		const res3 = await getRecords(token);
		setLoading(false);
		if (res.error || !res.result) {
			if (res.error) {
				notify(res.error.error, 'error');
			} else {
				notify('Jadwal tidak ditemukan', 'error');
			}
			setTimeout(() => {
				goto(DASHBOARD_SCHEDULE);
			}, 1000);
			return;
		}
		if (res2.result) {
			for (const id of res.result.device_ids) {
				const av = res2.result.find((a) => a.id === id);
				if (av) {
					avs.push({ name: av.address ?? av.unique_id, value: av.id });
				}
			}
		}
		if (res3.result) {
			const rec = res3.result.find((r) => r.id === res.result?.records_id);
			if (rec) {
				record = rec;
			}
		}
		schedule = res.result;
		for (const v of res.result.volumes) {
			const split = v.split(':');
			volumes[Number(split[0])] = Number(split[1]);
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
		for (const av of avs) {
			if (!added.includes(av.value)) {
				res.push(`${av.name}:1.0`);
			}
		}
		return res;
	};

	const submit = async () => {
		const token = $authStore.token || '';
		const volumes = getVolumes();
		setLoading(true);
		const res = await updateScheduleVolume(token, schedule!.id, volumes);
		setLoading(false);
		if (res.error || !res.result) {
			return notify(res.error?.error ?? 'Gagal mengupdate volume', 'error');
		}
		notify('Volume berhasil diperbarui', 'success');
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
	};

	onMount(() => {
		load();
	});
</script>

<DashboardInner title="Edit Jadwal" {breadcrumbs}>
	<div class="w-full flex-1 flex flex-col bg-white p-4 overflow-hidden">
		<div class="bg-gray-200 rounded-lg py-2 px-4 flex gap-2 items-center mb-4">
			<Announcement class="w-8 h-8" />
			<h1 class="font-semibold text-primary-600">
				Edit hanya dapat di lakukan untuk mengatur volume AVS
			</h1>
		</div>
		{#if schedule}
			<div class="flex-1 overflow-hidden">
				<div class="p-6 border border-gray-300 rounded-lg h-full overflow-y-auto">
					<h1 class="mb-8 text-xl font-semibold text-center">Data Jadwal</h1>
					<Label>
						Judul
						<Input type="text" placeholder="Masukan Judul" value={schedule.name} disabled />
					</Label>
					<div class="mb-4">
						Repetisi
						<div class="pl-4">
							<InputRadio items={repetitionOptions} selected={schedule.kind} disabled />
						</div>
					</div>
					<div class="mx-2 p-4 rounded-md bg-gray-100 mb-4">
						{#if schedule.kind === 1}
							<h1 class="mb-4 font-semibold">Jadwal Repetisi</h1>
							<Select
								items={repetitionType}
								selected={schedule.dates.length > 0 ? 1 : 2}
								label="Tipe Repetisi"
								placeholder="Pilih Tipe"
								disabled
							/>
							{#if schedule.dates.length > 0}
								<MultiDateOnlyPicker
									year={schedule.year || 0}
									month={schedule.month || 0}
									selected={schedule.dates}
									noheader
									disabled
								/>
							{:else}
								<Label>Minggu</Label>
								<div class="mt-4 flex gap-8 flex-wrap">
									<Label
										class="flex items-center gap-2 {schedule.weeks.length != 5 && 'text-gray-500'}"
									>
										<CheckBox checked={schedule.weeks.length == 5} disabled />
										Pilih Semua
									</Label>
									{#each weeks as week}
										{@const checked = schedule.weeks.includes(week)}
										<Label class="flex items-center gap-2 {!checked && 'text-gray-500'}">
											<CheckBox {checked} disabled />
											Minggu ke-{week}
										</Label>
									{/each}
								</div>
								<hr class="mb-2" />
								<Label>Hari</Label>
								<div class="mt-4 flex gap-8 flex-wrap">
									<Label
										class="flex items-center gap-2 {schedule.days.length != days.length &&
											'text-gray-500'}"
									>
										<CheckBox checked={schedule.days.length == days.length} disabled />
										Pilih Semua
									</Label>
									{#each days as day}
										{@const checked = schedule.days.includes(day.value)}
										<Label class="flex items-center gap-2 {!checked && 'text-gray-500'}">
											<CheckBox {checked} disabled />
											{day.name}
										</Label>
									{/each}
								</div>
							{/if}
						{:else}
							<h1 class="mb-4 font-semibold">Jadwal Non-Repetisi</h1>
							<Select
								items={[{ name: `${schedule.year}`, value: schedule.year || 0 }]}
								selected={schedule.year || 0}
								placeholder="Pilih tahun"
								label="Tahun"
								disabled
							/>
							<Select
								items={months}
								selected={schedule.month || 0}
								placeholder="Pilih bulan"
								label="Bulan"
								disabled
							/>
							<MultiDateOnlyPicker
								year={schedule.year || 0}
								month={schedule.month || 0}
								selected={schedule.dates}
								disabled
							/>
						{/if}
						<hr class="mb-2" />
						<Label>
							Plih Waktu
							<button
								class="flex justify-between items-center w-full text-start p-2 rounded bg-white border-gray-200 border"
							>
								<span>Pilih</span>
								<Clock class="w-6 h-6 fill-gray-500" />
							</button>
						</Label>
						{#if schedule.times.length > 0}
							<div class="flex gap-2 flex-wrap items-center">
								{#each schedule.times as time}
									<BadgeLabel name={time} bg="primary-400" rounded="md" />
								{/each}
							</div>
						{/if}
					</div>
					<SelectModal
						placeholder="Dapat pilih lebih dari satu"
						title="AVS"
						searchText="Cari AVS"
						items={avs}
						selected={avs}
						disabled
					/>
					{#if avs.length > 0}
						<div class="mx-2 p-4 rounded-md bg-gray-100 mb-4">
							<h1 class="mb-4 font-semibold">Atur Volume</h1>
							{#each avs as av}
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
					{#if record}
						<Select
							label="Rekaman"
							placeholder="Pilih Rekaman"
							items={[{ name: record.name, value: record.id }]}
							selected={record.id}
							disabled
						/>
					{/if}
					<div class="flex justify-end">
						<Button class="bg-primary-600 text-white" on:click={submit}>Kirim</Button>
					</div>
				</div>
			</div>
		{/if}
	</div>
</DashboardInner>
