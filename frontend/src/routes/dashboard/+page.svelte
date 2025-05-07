<script lang="ts">
	import Calendar from '$lib/components/ui/Calendar.svelte';
	import DashboardBox from '$lib/components/ui/DashboardBox.svelte';
	import { Icon } from '$lib/const/icon';
	import { getSchedule } from '$lib/service/schedule';
	import { getStatistics, getUsers } from '$lib/service/user';
	import { authStore, getToken } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import type { Avs, Records, Schedule, StatisticsRes, User } from '$lib/types/response';
	import { formatDurationToString } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import moment from 'moment';
	import { getTodaySchedule } from '$lib/utils/schedule';
	import { getAvs } from '$lib/service/avs';
	import { getRecords } from '$lib/service/record';

	let name = '';
	let statistics: StatisticsRes = {
		avs: 0,
		duration: 0,
		unit: 0,
		records: 0,
		schedule: 0
	};
	let schedules: Schedule[] = [];
	let records: Records[] = [];
	let avs: Avs[] = [];
	let users: User[] = [];

	onMount(async () => {
		let token = getToken();
		setLoading(true);
		const statisticsRes = await getStatistics(token || '');
		const scheduleRes = await getSchedule(token || '');
		const avsRes = await getAvs(token || '');
		const recordsRes = await getRecords(token || '');
		users = await getUsers(token || '');
		setLoading(false);
		if (statisticsRes.result) {
			statistics = statisticsRes.result;
		}
		if (scheduleRes.result) {
			schedules = scheduleRes.result;
		}
		if (avsRes.result) {
			avs = avsRes.result;
		}
		if (recordsRes.result) {
			records = recordsRes.result;
		}
	});

	authStore.subscribe((value) => {
		if (value.user) {
			name = value.user.name;
		}
	});

	$: todays = getTodaySchedule(schedules);
</script>

<div class="w-full p-2 md:p-4">
	<div class="mt-4">
		<h1 class="text-3xl font-bold">Selamat datang, {name}!</h1>
	</div>
	<div class="mt-8 w-full grid grid-cols-2 md:grid-cols-5 gap-2 md:gap-4">
		<DashboardBox
			icon={Icon.Clock}
			color="darkBlue"
			title="Jam Rekaman"
			value={formatDurationToString(statistics.duration)}
		/>
		<DashboardBox
			icon={Icon.Calendar}
			color="green"
			title="Jadwal"
			value={`${statistics.schedule}`}
		/>
		<DashboardBox icon={Icon.Music} color="blue" title="Rekaman" value={`${statistics.records}`} />
		<DashboardBox icon={Icon.Building} color="red" title="Unit" value={`${statistics.unit}`} />
		<DashboardBox
			icon={Icon.Server}
			color="yellow"
			title="AVS"
			value={`${statistics.avs}`}
			class="col-span-2 md:col-span-1"
		/>
	</div>
	<div class="mt-4 flex flex-col md:flex-row gap-4">
		<div class="w-full md:w-3/4">
			<Calendar {schedules} {records} {avs} {users} />
		</div>
		<div class="w-full md:w-1/4 pt-2">
			<h1 class="text-xl">Scheduled</h1>
			<div class="mt-3 p-2 rounded-md border border-gray-300 max-h-[500px] overflow-y-auto">
				<p class="my-4 font-bold">{moment().format('LL')}</p>
				{#each todays as schedule}
					{#each schedule.times as time}
						<div class="p-2 mb-2 rounded-md bg-primary-500 bg-opacity-30">
							<h1 class="text-md">{schedule.name}</h1>
							<p class="italic text-gray-600">{time}</p>
						</div>
					{/each}
				{/each}
			</div>
		</div>
	</div>
</div>
