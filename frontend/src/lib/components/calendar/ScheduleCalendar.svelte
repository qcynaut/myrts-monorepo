<script lang="ts">
	import '../../../styles/schedule_calendar.scss';
	import Calendar from '@event-calendar/core';
	import TimeGrid from '@event-calendar/time-grid';
	import DayGrid from '@event-calendar/day-grid';
	import List from '@event-calendar/list';
	import AngleLeft from '../icon/AngleLeft.svelte';
	import AngleRight from '../icon/AngleRight.svelte';
	import type { Avs, Records, Schedule } from '$lib/types/response';
	import { expandSchedule } from '$lib/utils/schedule';
	import Modal from '../modal/Modal.svelte';
	import Times from '../icon/Times.svelte';
	import CalendarIcon from '../icon/Calendar.svelte';
	import Exchange from '../icon/Excange.svelte';
	import Bullseye from '../icon/Bullseye.svelte';
	import Server from '../icon/Server.svelte';
	import VolumeUp from '../icon/VolumeUp.svelte';
	import Music from '../icon/Music.svelte';
	import { formatDurationToString } from '$lib/utils/format';
	import Pencil from '../icon/Pencil.svelte';
	import Trash from '../icon/Trash.svelte';
	import Button from '../button/Button.svelte';
	import { goto } from '$app/navigation';
	import { DASHBOARD_SCHEDULE } from '$lib/const/navigation';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import { deleteSchedule as delsec } from '$lib/service/schedule';
	import { notify } from '$lib/stores/toast';
	import Clock from '../icon/Clock.svelte';

	export let view: string = 'timeGridDay';
	export let dateRangeCallback: (start: Date, end: Date) => void = () => {};
	export let schedules: Schedule[] = [];
	export let records: Records[] = [];
	export let avs: Avs[] = [];
	export let load: () => Promise<void> = async () => {};

	let cal: HTMLDivElement;
	let monthTitle = new Date().toLocaleDateString('id', {
		month: 'long',
		year: 'numeric'
	});
	let moreModal = false;
	let moreEvents: HTMLDivElement;
	let moreTitle = '';

	let detailSchedule: Schedule | null = null;
	let detailEvent: any | null = null;
	let deleteSchedule: Schedule | null = null;

	const currentScroll = () => {
		const now = new Date();
		return `${now.getHours()}:${now.getMinutes()}:${now.getSeconds()}`;
	};

	let events = expandSchedule(new Date(), new Date(), schedules, records);
	let plugins = [TimeGrid, DayGrid, List];
	let options = {
		view,
		locale: 'id',
		allDaySlot: false,
		headerToolbar: { start: 'prev,next', center: '', end: '' },
		height: '100%',
		slotDuration: '00:00:30',
		dayMaxEvents: true,
		nowIndicator: true,
		scrollTime: currentScroll(),
		titleFormat: (start: Date, end: Date) => {
			dateRangeCallback(start, end);
			if (view === 'dayGridMonth') {
				monthTitle = start.toLocaleDateString('id', {
					month: 'long',
					year: 'numeric'
				});
			}
		},
		dayHeaderFormat: (date: Date) => {
			if (view === 'dayGridMonth' || view === 'timeGridDay') {
				return date.toLocaleDateString('id', {
					weekday: 'long'
				});
			} else {
				const dateNum = date.getDate();
				const weekday = date.toLocaleDateString('id', {
					weekday: 'long'
				});
				return {
					html: `<div class="cal-header"><div class="cal-days"><span class="cal-date">${dateNum}</span><span class="cal-week">${weekday}</span></div></div>`
				};
			}
		},
		events,
		eventClassNames: (info: any) => {
			const classNames = ['events'];
			const event = info.event;
			if (event.id && event.id.split('_').includes('rep')) {
				classNames.push('repetition');
			}
			classNames.push(`event_id_${event.id}`);
			return classNames;
		},
		moreLinkContent: (arg: any) => {
			const btn = document.createElement('button');
			btn.innerHTML = arg.text;
			btn.onclick = () => {
				setTimeout(() => {
					const popup = document.querySelector('.ec-popup');
					if (popup) {
						const eventsel = popup.querySelectorAll('.ec-event');
						const header = popup.querySelector('.ec-day-head span');
						moreTitle = header?.innerHTML ?? '';
						eventsel.forEach((event, i) => {
							event.addEventListener('click', () => {
								if (moreEvents) {
									moreEvents.childNodes.forEach((c) => {
										c.remove();
									});
								}
								moreModal = false;
								const classWithId = Array.from(event.classList).find((c) => c.includes('event_id'));
								if (classWithId) {
									const id = classWithId.replace('event_id_', '').split('_')[0];
									const schedule = schedules.find((s) => s.id === parseInt(id));
									let ev = events.find((e) => e.id === classWithId.replace('event_id_', ''));
									if (ev) {
										detailEvent = ev;
									}
									if (schedule) {
										detailSchedule = schedule;
									}
								}
							});
							const div = document.createElement('div');
							div.classList.add('more-ct');
							const index = document.createElement('span');
							index.innerHTML = (i + 1).toString();
							div.appendChild(index);
							div.appendChild(event);
							moreEvents?.appendChild(div);
						});
						moreModal = true;
						popup.remove();
					}
				}, 100);
			};
			return {
				domNodes: [btn]
			};
		},
		eventClick: (info: any) => {
			const eid = info.event.id;
			const sid = eid.split('_')[0];
			detailEvent = info.event;
			const schedule = schedules.find((s) => s.id === parseInt(sid));
			if (schedule) {
				detailSchedule = schedule;
			}
		},
		datesSet: (info: any) => {
			events = expandSchedule(info.start, info.end, schedules, records);
			options.events = events;
		},
		noEventsContent: 'Tidak ada jadwal untuk minggu ini'
	};

	$: {
		options = {
			...options,
			view
		};
	}

	const createNavigation = () => {
		const btnGroup = cal.querySelector('.ec-button-group');
		let sidebar = cal.querySelector('.ec-sidebar');
		if (!sidebar) {
			setTimeout(() => {
				createNavigation();
			}, 200);
			return;
		} else if (cal.querySelector('#cal-nav')) {
			return;
		}
		const prev_orig = btnGroup?.querySelector('.ec-prev');
		const next_orig = btnGroup?.querySelector('.ec-next');
		const div = document.createElement('div');
		div.id = 'cal-nav';
		const prev = document.createElement('button');
		const next = document.createElement('button');
		prev.innerHTML = prev_orig?.innerHTML ?? '';
		next.innerHTML = next_orig?.innerHTML ?? '';
		prev.onclick = () => {
			if (prev_orig) {
				const btn = prev_orig as HTMLButtonElement;
				btn.click();
			}
		};
		next.onclick = () => {
			if (next_orig) {
				const btn = next_orig as HTMLButtonElement;
				btn.click();
			}
		};
		div.appendChild(prev);
		div.appendChild(next);
		sidebar?.children[0].insertAdjacentElement('afterend', div);
	};

	const prevMonth = () => {
		const btnGroup = cal.querySelector('.ec-button-group');
		const prev = btnGroup?.querySelector('.ec-prev');
		if (prev) {
			const btn = prev as HTMLButtonElement;
			btn.click();
		}
	};
	const nextMonth = () => {
		const btnGroup = cal.querySelector('.ec-button-group');
		const next = btnGroup?.querySelector('.ec-next');
		if (next) {
			const btn = next as HTMLButtonElement;
			btn.click();
		}
	};

	const dayMap = ['Minggu', 'Senin', 'Selasa', 'Rabu', 'Kamis', 'Jumat', 'Sabtu'];
	const monthMap = [
		'Januari',
		'Februari',
		'Maret',
		'April',
		'Mei',
		'Juni',
		'Juli',
		'Agustus',
		'September',
		'Oktober',
		'November',
		'Desember'
	];

	const floatToPercentage = (num: string) => {
		const n = parseFloat(num) * 100;
		// sometimse it's become 110.000000000001
		// we need only the number on the left
		return n.toString().split('.')[0];
	};

	const deleteScheduleAction = async () => {
		if (!deleteSchedule) return;
		const token = $authStore.token || '';
		const id = deleteSchedule.id;
		deleteSchedule = null;
		setLoading(true);
		const res = await delsec(token, id);
		setLoading(false);
		if (res.error) {
			notify(res.error.error, 'error');
		} else {
			notify('Jadwal berhasil di hapus', 'success');
			window.location.reload();
		}
	};

	$: {
		let cond = cal && !cal?.querySelector('#cal-nav');
		if (view === 'dayGridMonth') {
			cal?.querySelector('#cal-nav')?.remove();
			cond = false;
		}
		if (cond) {
			createNavigation();
		}
	}
</script>

<Modal
	bind:open={moreModal}
	class="w-3/4 md:w-2/3 h-2/3 overflow-hidden flex flex-col"
	closable={false}
>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span class="font-semibold">Daftar Jadwal {moreTitle}</span>
		<button
			class="p-2 rounded-full bg-white bg-opacity-25"
			on:click={() => {
				if (moreEvents) moreEvents.innerHTML = '';
				moreModal = false;
			}}
		>
			<Times class="w-4 h-4 fill-white" />
		</button>
	</div>
	<div class="more-events" bind:this={moreEvents} />
</Modal>

<Modal open={deleteSchedule !== null} closable={false} class="p-8">
	<h1 class="text-center text-xl font-semibold mb-2">Peringatan</h1>
	<div class="text-center mb-6">
		Apakah kamu yakin ingin menghapus <span class="font-semibold">{deleteSchedule?.name}</span>
	</div>
	<div class="flex justify-end items-center gap-2">
		<Button
			class="bg-none text-primary-600 border border-primary-600"
			on:click={() => (deleteSchedule = null)}>Batal</Button
		>
		<Button class="bg-red-600 text-white" on:click={deleteScheduleAction}>Hapus</Button>
	</div>
</Modal>

<Modal
	open={detailSchedule !== null}
	class="w-3/4 md:w-1/3 md:h-3/4 overflow-hidden flex flex-col"
	closable={false}
>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span class="font-semibold">Detail Jadwal</span>
		<div class="flex items-center gap-2">
			<button
				class="p-2 rounded-full bg-white bg-opacity-25"
				on:click={() => {
					goto(`${DASHBOARD_SCHEDULE}/edit/${detailSchedule?.id}`);
				}}
			>
				<Pencil class="w-4 h-4 fill-white" />
			</button>
			<button
				class="p-2 rounded-full bg-white bg-opacity-25"
				on:click={() => {
					deleteSchedule = detailSchedule;
					detailSchedule = null;
					detailEvent = null;
				}}
			>
				<Trash class="w-4 h-4 fill-white" />
			</button>
			<button
				class="p-2 rounded-full bg-white bg-opacity-25"
				on:click={() => (detailSchedule = null)}
			>
				<Times class="w-4 h-4 fill-white" />
			</button>
		</div>
	</div>
	{#if detailSchedule}
		<div class="flex-1 overflow-y-auto p-4">
			<div class="flex items-center gap-2 border-b border-gray-200 border-dashed pb-4 mb-4">
				<CalendarIcon class="w-8 h-8 fill-primary-600" />
				<h1 class="text-lg font-semibold">{detailSchedule.name}</h1>
			</div>
			{#if detailEvent}
				<div class="flex items-center gap-2 mb-4">
					<Clock class="w-6 h-6 fill-primary-600" />
					<h1 class="text-sm">
						{detailEvent.start.getHours()}:{detailEvent.start
							.getMinutes()
							.toString()
							.padStart(2, '0')}
					</h1>
				</div>
				<div class="flex items-center gap-2 mb-4">
					<CalendarIcon class="w-6 h-6 fill-primary-600" />
					<h1 class="text-sm">
						{detailEvent.start.toLocaleDateString('id', {
							weekday: 'long',
							day: 'numeric',
							month: 'long',
							year: 'numeric'
						})}
					</h1>
				</div>
				<hr class="mb-4" />
			{/if}
			<div class="flex items-center gap-2 border-b border-gray-200 border-dashed pb-4 mb-4 text-sm">
				<Exchange class="w-6 h-6 fill-primary-600" />
				{#if detailSchedule.kind === 1}
					{#if detailSchedule.weeks.length > 1}
						<span>Repetisi Mingguan</span>
					{:else}
						<span>Repetisi Bulanan</span>
					{/if}
				{:else}
					<span>Non-repetisi</span>
				{/if}
			</div>
			{#if detailSchedule.weeks.length > 0}
				<div class="flex items-center text-sm gap-4 pl-4 text-gray-400">
					<div class="w-[15%] border-r border-r-gray-400 py-2">Minggu</div>
					<div class="w-[85%] flex gap-2 flex-wrap items-center py-2">
						{#each detailSchedule.weeks as week}
							<span class="non-repetition-bg text-non-repetition py-1 px-2 rounded-full"
								>{week}</span
							>
						{/each}
					</div>
				</div>
			{/if}
			{#if detailSchedule.days.length > 0}
				<div class="flex items-center text-sm gap-4 mb-4 pl-4 text-gray-400">
					<div class="w-[15%] border-r border-r-gray-400 py-2">Hari</div>
					<div class="w-[85%] flex gap-2 flex-wrap items-center py-2">
						{#each detailSchedule.days as day}
							<span class="non-repetition-bg text-non-repetition py-1 px-2 rounded-full"
								>{dayMap[day - 1]}</span
							>
						{/each}
					</div>
				</div>
			{/if}
			{#if detailSchedule.year}
				<div class="flex items-center text-sm gap-4 mb-4 pl-4 text-gray-400">
					<div class="w-[15%] border-r border-r-gray-400 py-2">Tahun</div>
					<div class="w-[85%] flex gap-2 flex-wrap items-center py-2">
						<span class="non-repetition-bg text-non-repetition py-1 px-2 rounded-full"
							>{detailSchedule.year}</span
						>
					</div>
				</div>
			{/if}
			{#if detailSchedule.month}
				<div class="flex items-center text-sm gap-4 mb-4 pl-4 text-gray-400">
					<div class="w-[15%] border-r border-r-gray-400 py-2">Month</div>
					<div class="w-[85%] flex gap-2 flex-wrap items-center py-2">
						<span class="non-repetition-bg text-non-repetition py-1 px-2 rounded-full"
							>{monthMap[detailSchedule.month - 1]}</span
						>
					</div>
				</div>
			{/if}
			{#if detailSchedule.dates.length > 0}
				<div class="flex items-center text-sm gap-4 mb-4 pl-4 text-gray-400">
					<div class="w-[15%] border-r border-r-gray-400 py-2">Tanggal</div>
					<div class="w-[85%] flex gap-2 flex-wrap items-center py-2">
						{#each detailSchedule.dates as date}
							<span class="non-repetition-bg text-non-repetition py-1 px-2 rounded-full"
								>{date}</span
							>
						{/each}
					</div>
				</div>
			{/if}
			<hr class="my-4" />
			<div class="flex gap-2 items-center border-b border-gray-200 border-dashed pb-4 mb-4">
				<Bullseye class="w-6 h-6 fill-primary-600" />
				<span class="text-sm">AVS</span>
			</div>
			<div class="pl-4 flex flex-col gap-2 text-sm">
				{#each detailSchedule.device_ids as did}
					{@const av = avs.find((av) => av.id === did)}
					{#if av}
						<div class="flex gap-2 items-center">
							<Server class="w-4 h-4 fill-primary-600" />
							<span>{av.address || av.unique_id}</span>
							<VolumeUp class="w-4 h-4 fill-primary-600" />
							<span
								>{floatToPercentage(
									detailSchedule.volumes.find((v) => v.includes(`${did}:`))?.split(':')[1] || '1.0'
								)}%</span
							>
						</div>
					{/if}
				{/each}
			</div>
			{#if records.find((r) => r.id === detailSchedule?.records_id)}
				{@const record = records.find((r) => r.id === detailSchedule?.records_id)}
				<hr class="my-4" />
				<div class="flex gap-2 items-center border-b border-gray-200 border-dashed pb-4 mb-4">
					<Music class="w-6 h-6 fill-primary-600" />
					<span class="text-sm">Rekaman</span>
				</div>
				<div class="flex gap-2 pl-4 text-sm">
					<span>{record?.name || '-'}</span>
					-
					<span class="text-gray-500"
						>{formatDurationToString(parseInt(record?.duration || '0'))}</span
					>
				</div>
			{/if}
		</div>
	{/if}
</Modal>

<div class="w-full h-full flex flex-col" bind:this={cal}>
	<div class="flex justify-center items-center gap-4 pt-2">
		<div class="flex gap-2 items-center">
			<span class="w-4 h-4 rounded-full repetition-bg" />
			<span class="text-sm text-gray-400 font-semibold">Jadwal repetisi</span>
		</div>
		<div class="flex gap-2 items-center">
			<span class="w-4 h-4 rounded-full non-repetition-bg" />
			<span class="text-sm text-gray-400 font-semibold">Jadwal non-repetisi</span>
		</div>
	</div>
	{#if view === 'dayGridMonth'}
		<div class="flex w-full justify-center items-center gap-2 pt-2">
			<button on:click={prevMonth}><AngleLeft class="w-4 h-4 fill-black" /></button>
			<h1 class="text-xl font-semibold">{monthTitle}</h1>
			<button on:click={nextMonth}><AngleRight class="w-4 h-4 fill-black" /></button>
		</div>
	{/if}
	<div class="w-full h-full flex-1 overflow-x-auto pt-4">
		<div class="w-full h-full">
			<Calendar {plugins} {options} />
		</div>
	</div>
</div>
