<script lang="ts">
	import type { Instance } from 'flatpickr/dist/types/instance';
	import Calendar from '../icon/Calendar.svelte';
	import Label from './Label.svelte';
	import flatpickr from 'flatpickr';
	import { beforeUpdate, onMount } from 'svelte';
	import 'flatpickr/dist/themes/material_blue.css';
	import '../../../styles/datepicker.scss';
	import BadgeLabel from '../badge/BadgeLabel.svelte';

	export let year: number = new Date().getFullYear();
	export let month: number = new Date().getMonth() + 1;
	export let selected: number[] = [];
	export let noheader = false;
	export let disabled: boolean = false;
	let fpValue: string = '';

	let elem: HTMLInputElement;
	let fp: Instance | undefined = undefined;

	const createFp = () => {
		const date = new Date();
		date.setFullYear(year);
		date.setMonth(month - 1);
		if (noheader) {
			date.setFullYear(2023);
			date.setMonth(9);
		}
		const firstDay = new Date(date.getFullYear(), date.getMonth(), 1);
		const lastDay = new Date(date.getFullYear(), date.getMonth() + 1, 0);
		fp = flatpickr(elem, {
			enableTime: false,
			mode: 'multiple',
			dateFormat: 'd',
			conjunction: '|',
			minDate: firstDay,
			maxDate: lastDay
		});
	};

	onMount(() => {
		createFp();
		if (noheader) {
			fp?.calendarContainer.classList.add('noheader');
		}
	});

	beforeUpdate(() => {
		const date = new Date();
		date.setFullYear(year);
		date.setMonth(month - 1);
		if (noheader) {
			date.setFullYear(2023);
			date.setMonth(9);
		}
		const firstDay = new Date(date.getFullYear(), date.getMonth(), 1);
		const lastDay = new Date(date.getFullYear(), date.getMonth() + 1, 0);
		fp?.set({
			minDate: firstDay,
			maxDate: lastDay
		});
	});

	const remove = (item: number) => {
		selected = selected.filter((i) => i !== item);
		const arr = selected.join('|');
		fpValue = arr;
	};

	$: {
		if (fpValue.includes('|') || fpValue !== '') {
			const arr = fpValue.split('|');
			selected = arr.map((item) => parseInt(item));
		}
	}
</script>

<Label>
	Pilih Tanggal
	<input bind:this={elem} class="absolute opacity-0" bind:value={fpValue} {disabled} />
	<div class="relative w-full p-2 border border-gray-200 rounded-md bg-white">
		{#if selected.length === 0}
			<span class="text-gray-500">Pilih Tanggal</span>
		{:else}
			<div class="pr-6 flex gap-2 items-center flex-wrap">
				{#each selected as item}
					<BadgeLabel
						name={item.toString()}
						bg="primary-400"
						rounded="md"
						on:close={() => {
							if (!disabled) remove(item);
						}}
						padding="p-1"
					/>
				{/each}
			</div>
		{/if}
		<Calendar class="w-6 h-6 fill-gray-500 absolute right-2 top-1/2 transform -translate-y-1/2" />
	</div>
</Label>
