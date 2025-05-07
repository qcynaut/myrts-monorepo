<script lang="ts">
	import flatpickr from 'flatpickr';
	import 'flatpickr/dist/themes/material_blue.css';
	import '../../../styles/datepicker.scss';
	import { onMount } from 'svelte';
	import { Label, type FormSizeType, Badge, CloseButton } from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';
	import type { Instance } from 'flatpickr/dist/types/instance';

	export let label: string = '';
	export let value: number[] = [];
	export let disabled: boolean = false;
	export let size: FormSizeType = 'md';
	export let onlyDate = false;
	export let month = new Date().getMonth() + 1;
	export let year = new Date().getFullYear();

	let fp: Instance | undefined = undefined;
	let elem: HTMLInputElement;
	let dvalue = '';

	const modalSelectClass: string =
		'relative border border-gray-300 flex items-center rounded-lg gap-2 dark:border-gray-600 focus-within:ring-1 focus-within:border-primary-500 ring-primary-500 dark:focus-within:border-primary-500 dark:ring-primary-500 min-h-[40px]';
	const sizes = {
		sm: 'px-2 py-1',
		md: 'px-3 py-2',
		lg: 'px-4 py-3'
	};

	const filterMonth = () => {
		const date = new Date();
		date.setFullYear(year);
		date.setMonth(month - 1);
		// get 1'st date of the month
		const firstDay = new Date(date.getFullYear(), date.getMonth(), 1);
		// get last date of the month
		const lastDay = new Date(date.getFullYear(), date.getMonth() + 1, 0);
		return { from: firstDay, to: lastDay };
	};

	onMount(() => {
		fp = flatpickr(elem, {
			enableTime: false,
			mode: 'multiple',
			dateFormat: 'd',
			conjunction: '|'
		});
		if (onlyDate) {
			const { from: minDate, to: maxDate } = filterMonth();
			fp.set({
				minDate: minDate,
				maxDate: maxDate
			});
		}
	});

	const onblur = () => {
		if (fp) {
			if (onlyDate) {
				const { from: minDate, to: maxDate } = filterMonth();
				fp.set({
					minDate: minDate,
					maxDate: maxDate
				});
			}
		}

		alert(dvalue);
	};

	$: value = dvalue != '' ? dvalue.split('|').map(Number) : [];
</script>

<div class="mb-6 relative">
	<Label>
		{label}
		<input
			type="text"
			bind:value={dvalue}
			bind:this={elem}
			on:blur={onblur}
			{disabled}
			class="absolute bottom-0 left-1/2 transform -translate-x-1/2 opacity-0 h-0 -z-10"
		/>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			on:click={() => {
				if (disabled) return;
			}}
			tabindex="-1"
			role="listbox"
			class={twMerge(modalSelectClass, sizes[size], $$props.class)}
		>
			<span class="flex gap-2 flex-wrap">
				{#if value.length}
					{#each value as item}
						<slot {item} clear={() => {}}>
							<Badge
								color="primary"
								large={size === 'lg'}
								params={{ duration: 100 }}
								on:close={() => {}}
							>
								{item}
							</Badge>
						</slot>
					{/each}
				{/if}
			</span>
			<div class="flex ml-auto gap-2 items-center">
				{#if value.length}
					<CloseButton
						on:click={(e) => {
							e.stopPropagation();
							e.preventDefault();
							if (fp) fp.clear();
						}}
						color="none"
						class="p-0 focus:ring-gray-400"
					/>
				{/if}
				<div class="w-[1px] bg-gray-300 dark:bg-gray-600" />
			</div>
		</div>
	</Label>
</div>
