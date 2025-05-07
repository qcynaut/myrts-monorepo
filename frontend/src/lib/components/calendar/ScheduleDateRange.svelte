<script lang="ts">
	import Calendar from '$lib/components/icon/Calendar.svelte';
	import { onMount } from 'svelte';

	export let view = 'timeGridDay';
	let display = '';

	export const setDates = (start: Date, end: Date) => {
		if (view === 'timeGridDay') {
			display = start.toLocaleDateString('id', {
				weekday: 'long',
				year: 'numeric',
				month: 'long',
				day: 'numeric'
			});
		} else if (view === 'dayGridMonth') {
			display = start.toLocaleDateString('id', {
				month: 'long',
				year: 'numeric'
			});
		} else if (view === 'timeGridWeek') {
			const startYear = start.getFullYear();
			const endYear = end.getFullYear();
			const startMonth = start.getMonth();
			const endMonth = end.getMonth();
			if (startYear == endYear) {
				if (startMonth == endMonth) {
					const startStr = start.toLocaleDateString('id', {
						day: 'numeric'
					});
					const endStr = end.toLocaleDateString('id', {
						day: 'numeric',
						month: 'long',

						year: 'numeric'
					});
					display = `${startStr} - ${endStr}`;
				} else {
					const startStr = start.toLocaleDateString('id', {
						day: 'numeric',
						month: 'short'
					});
					const endStr = end.toLocaleDateString('id', {
						day: 'numeric',
						month: 'short',
						year: 'numeric'
					});
					display = `${startStr} - ${endStr}`;
				}
			} else {
				const startStr = start.toLocaleDateString('id', {
					day: 'numeric',
					month: 'short',
					year: 'numeric'
				});
				const endStr = end.toLocaleDateString('id', {
					day: 'numeric',
					month: 'short',
					year: 'numeric'
				});
				display = `${startStr} - ${endStr}`;
			}
		}
	};

	onMount(() => {
		setDates(new Date(), new Date());
	});
</script>

<div
	class="bg-white text-black flex justify-between items-center p-2 rounded-md border border-gray-200"
>
	<div class="flex items-center gap-4 mr-8">
		<span class="text-black font-semibold">{display}</span>
	</div>
	<Calendar class="w-4 h-4 fill-black" />
</div>
