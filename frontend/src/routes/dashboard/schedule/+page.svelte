<script lang="ts">
	import { goto } from '$app/navigation';
	import Button from '$lib/components/button/Button.svelte';
	import ScheduleCalendar from '$lib/components/calendar/ScheduleCalendar.svelte';
	import ScheduleDateRange from '$lib/components/calendar/ScheduleDateRange.svelte';
	import Plus from '$lib/components/icon/Plus.svelte';
	import EmptySchedule from '$lib/components/images/EmptySchedule.svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import Tabs from '$lib/components/tabs/Tabs.svelte';
	import { DASHBOARD_SCHEDULE_ADD } from '$lib/const/navigation';
	import { getAvs } from '$lib/service/avs';
	import { getRecords } from '$lib/service/record';
	import { getSchedule } from '$lib/service/schedule';
	import { authStore } from '$lib/stores/auth';
	import { notify } from '$lib/stores/toast';
	import { setLoading } from '$lib/stores/ui';
	import type { Avs, Records, Schedule } from '$lib/types/response';
	import { onMount } from 'svelte';

	const calendarGrid = [
		{
			name: 'Harian',
			value: 'timeGridDay'
		},
		{
			name: 'Mingguan',
			value: 'timeGridWeek'
		},
		{
			name: 'Bulanan',
			value: 'dayGridMonth'
		},
		{
			name: 'List',
			value: 'listWeek'
		}
	];
	let activeGrid = 'timeGridDay';

	let dateRange: ScheduleDateRange;

	const setDateRange = (start: Date, end: Date) => {
		if (dateRange) {
			dateRange.setDates(start, end);
		}
	};

	let schedules: Schedule[] = [];
	let records: Records[] = [];
	let avs: Avs[] = [];

	const load = async () => {
		const token = $authStore.token || '';
		setLoading(true);
		const schedulesRes = await getSchedule(token);
		const recordsRes = await getRecords(token);
		const avsRes = await getAvs(token);
		setLoading(false);
		if (schedulesRes.error) {
			notify(schedulesRes.error.error, 'error');
		}
		if (recordsRes.error) {
			notify(recordsRes.error.error, 'error');
		}
		if (avsRes.error) {
			notify(avsRes.error.error, 'error');
		}
		if (schedulesRes.result) {
			schedules = schedulesRes.result;
		}
		if (recordsRes.result) {
			records = recordsRes.result;
		}
		if (avsRes.result) {
			avs = avsRes.result;
		}
	};

	onMount(() => {
		load();
	});
</script>

<DashboardInner title="Jadwal" subtitle="Daftar Jadwal">
	<Button
		class="bg-primary-600 text-white flex items-center gap-2"
		slot="endItem"
		on:click={() => goto(DASHBOARD_SCHEDULE_ADD)}
	>
		<Plus class="w-6 h-6 fill-white" />
		<span>Tambah Jadwal</span>
	</Button>
	<div class="w-full flex-1 bg-white p-4 overflow-hidden">
		<div class="w-full h-full flex flex-col overflow-hidden gap-2">
			<div class="flex flex-col md:flex-row md:justify-between items-center">
				<Tabs items={calendarGrid} bind:active={activeGrid} containerClass="bg-tertiary" />
				<ScheduleDateRange bind:this={dateRange} view={activeGrid} />
			</div>
			<div class="flex flex-col flex-1 h-2 overflow-y-hidden">
				{#if schedules.length === 0}
					<div class="flex items-center justify-center h-full">
						<div class="flex flex-col items-center">
							<EmptySchedule width={200} />
							<h1 class="font-semibold text-xl">Tidak Ada Jadwal Ditampilkan</h1>
						</div>
					</div>
				{:else}
					<ScheduleCalendar
						bind:view={activeGrid}
						dateRangeCallback={setDateRange}
						{schedules}
						{records}
						{avs}
						{load}
					/>
				{/if}
			</div>
		</div>
	</div>
</DashboardInner>
