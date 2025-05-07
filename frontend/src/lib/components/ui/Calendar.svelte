<script lang="ts">
	import Calendar from '@event-calendar/core';
	import TimeGrid from '@event-calendar/time-grid';
	import DayGrid from '@event-calendar/day-grid';
	import '../../../styles/calendar.scss';
	import { Button, Dropdown, DropdownDivider, DropdownItem, Modal } from 'flowbite-svelte';
	import { ChevronDownOutline, CalendarWeekOutline } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import type { Avs, Records, Schedule, User } from '$lib/types/response';
	import { expandSchedule } from '$lib/utils/schedule';
	import type { CalendarEvents, CalendarOptions, DateSetParams } from '$lib/types/calendar';

	export let schedules: Schedule[];
	export let records: Records[] = [];
	export let avs: Avs[] = [];
	export let users: User[] = [];

	let events: CalendarEvents[] = [];
	let calendar: HTMLElement;
	let timeline: HTMLElement;
	let dropdown: HTMLElement;
	let clickSchedule: Schedule | null = null;

	let plugins = [TimeGrid, DayGrid];
	let options: CalendarOptions = {
		view: 'dayGridMonth',
		allDaySlot: false,
		dayHeaderFormat: { weekday: 'short' },
		headerToolbar: { start: '', center: '', end: 'title prev,next' },
		locale: 'id',
		slotDuration: '00:15:00',
		datesSet: (info) => {
			events = expandSchedule(info.start, info.end, schedules, records);
		},
		dayMaxEvents: true,
		events: events,
		displayEventEnd: true,
		eventClassNames: (info) => {
			const classNames = ['events'];
			const event = info.event;
			if (event.id && event.id.split('_').includes('rep')) {
				classNames.push('rep');
			}
			return classNames;
		},
		eventClick: (info) => {
			const event = info.event;
			const id = parseInt(event.id.split('_')[0]);
			const schedule = schedules.find((s) => s.id === id);
			if (schedule) {
				clickSchedule = schedule;
			}
		}
	};

	const dayMap = [
		{ id: 1, name: 'Minggu' },
		{ id: 2, name: 'Senin' },
		{ id: 3, name: 'Selasa' },
		{ id: 4, name: 'Rabu' },
		{ id: 5, name: 'Kamis' },
		{ id: 6, name: 'Jumat' },
		{ id: 7, name: 'Sabtu' }
	];

	const updateType = (type: 'dayGridMonth' | 'timeGridWeek') => {
		options.view = type;
		if (type === 'dayGridMonth') {
			options.dayHeaderFormat = { weekday: 'short' };
		} else {
			options.dayHeaderFormat = { weekday: 'short', day: 'numeric' };
		}
	};

	onMount(() => {
		if (!browser) return;
		let toolbarEl = calendar.querySelector('.ec-toolbar');
		let firstDiv = toolbarEl?.childNodes[0];
		firstDiv?.appendChild(timeline);
		timeline.style.marginTop = '8px';
		let lastDiv = toolbarEl?.childNodes[toolbarEl.childNodes.length - 1];
		let firstChild = lastDiv?.childNodes[0];
		lastDiv?.insertBefore(dropdown, firstChild!);
		dropdown.style.marginTop = '8px';
		events = expandSchedule(new Date(), new Date(), schedules, records);
	});

	let dropdownOpen = false;

	$: events = expandSchedule(new Date(), new Date(), schedules, records);
	$: {
		options.events = events;
	}
</script>

<Modal open={clickSchedule !== null} size="md" on:close={() => (clickSchedule = null)}>
	<h1 class="font-semibold">Detail Jadwal</h1>
	<div class="flex gap-4 flex-col md:flex-row">
		<div class="mb-4 w-full md:w-1/2">
			<h1 class="text-lg mb-2">Nama jadwal</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				{clickSchedule?.name}
			</div>
		</div>
		<div class="mb-4 w-full md:w-1/2">
			<h1 class="text-lg mb-2">Tipe jadwal</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				{clickSchedule?.kind === 1 ? 'Repetisi' : 'Non Repetisi'}
			</div>
		</div>
	</div>
	{#if clickSchedule && clickSchedule.weeks.length > 0 && clickSchedule.days.length > 0}
		<div class="flex gap-4 flex-col md:flex-row">
			<div class="mb-4 w-full md:w-1/2">
				<h1 class="text-lg mb-2">Minggu ke</h1>
				<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center flex-wrap">
					{#each clickSchedule.weeks as week}
						<span class="p-2 text-xs rounded-l-full rounded-r-full bg-primary-500 text-white">
							ke-{week}
						</span>
					{/each}
				</div>
			</div>
			<div class="mb-4 w-full md:w-1/2">
				<h1 class="text-lg mb-2">Hari</h1>
				<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center flex-wrap">
					{#each clickSchedule.days as day}
						<span class="p-2 text-xs rounded-l-full rounded-r-full bg-primary-500 text-white">
							{dayMap.find((d) => d.id === day)?.name}
						</span>
					{/each}
				</div>
			</div>
		</div>
	{/if}
	{#if clickSchedule && clickSchedule.year && clickSchedule.year}
		<div class="flex gap-4 flex-col md:flex-row">
			<div class="mb-4 w-full md:w-1/2">
				<h1 class="text-lg mb-2">Tahun</h1>
				<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
					{clickSchedule.year}
				</div>
			</div>
			<div class="mb-4 w-full md:w-1/2">
				<h1 class="text-lg mb-2">Bulan</h1>
				<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
					{clickSchedule.month}
				</div>
			</div>
		</div>
	{/if}
	{#if clickSchedule && clickSchedule.dates.length > 0}
		<div class="mb-4">
			<h1 class="text-lg mb-2">Tanggal</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center flex-wrap">
				{#each clickSchedule.dates as date}
					<span class="p-2 text-xs rounded-l-full rounded-r-full bg-primary-500 text-white">
						{date}
					</span>
				{/each}
			</div>
		</div>
	{/if}
	<div class="mb-4">
		<h1 class="text-lg mb-2">Jam</h1>
		<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center flex-wrap">
			{#if clickSchedule}
				{#each clickSchedule.times as time}
					<span class="p-2 text-xs rounded-l-full rounded-r-full bg-primary-500 text-white">
						{time}
					</span>
				{/each}
			{/if}
		</div>
	</div>
	<div class="mb-4">
		<h1 class="text-lg mb-2">Pemilik</h1>
		<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
			{users.find((u) => u.id === clickSchedule?.user_id)?.name}
		</div>
	</div>
	<div class="mb-4">
		<h1 class="text-lg mb-2">AVS terpilih</h1>
		<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center flex-wrap">
			{#if clickSchedule}
				{#each clickSchedule.device_ids as id}
					{@const av = avs.find((a) => a.id === id)}
					<span class="p-2 text-xs rounded-l-full rounded-r-full bg-primary-500 text-white">
						{av?.address || av?.unique_id}
					</span>
				{/each}
			{/if}
		</div>
	</div>
	<div class="mb-4">
		<h1 class="text-lg mb-2">Rekaman</h1>
		<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
			{records.find((r) => r.id === clickSchedule?.records_id)?.name}
		</div>
	</div>
</Modal>

<div class="relative calendar h-full" bind:this={calendar}>
	<div class="flex items-center" bind:this={timeline}>
		<h1 class="hidden md:block text-xl">Timeline</h1>
	</div>
	<div bind:this={dropdown}>
		<Button class="bg-white text-gray-500 text-xs border border-gray-300 p-2 hover:bg-white"
			><CalendarWeekOutline class="w-3 h-3 mr-2" />{options.view === 'timeGridWeek'
				? 'Week'
				: 'Month'}<ChevronDownOutline class="w-3 h-3 ml-2" /></Button
		>
		<Dropdown bind:open={dropdownOpen}>
			<DropdownItem
				on:click={() => {
					updateType('timeGridWeek');
					dropdownOpen = false;
				}}>Week</DropdownItem
			>
			<DropdownDivider />
			<DropdownItem
				on:click={() => {
					updateType('dayGridMonth');
					dropdownOpen = false;
				}}>Month</DropdownItem
			>
		</Dropdown>
	</div>
	<Calendar {plugins} {options} />
</div>
