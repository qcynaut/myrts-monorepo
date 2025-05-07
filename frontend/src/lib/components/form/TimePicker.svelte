<script lang="ts">
	import { Label, type FormSizeType, Badge, CloseButton } from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';
	import { TimePicker } from 'svelte-time-picker';
	import Icons from '../ui/Icons.svelte';
	import { Icon } from '$lib/const/icon';
	import '../../../styles/timepicker.scss';

	export let label: string = '';
	export let value: string[] = [];
	export let disabled: boolean = false;
	export let size: FormSizeType = 'md';
	let open = false;

	const modalSelectClass: string =
		'relative border border-gray-300 flex items-center rounded-lg gap-2 dark:border-gray-600 focus-within:ring-1 focus-within:border-primary-500 ring-primary-500 dark:focus-within:border-primary-500 dark:ring-primary-500 min-h-[40px]';
	const sizes = {
		sm: 'px-2 py-1',
		md: 'px-3 py-2',
		lg: 'px-4 py-3'
	};

	const save = (e: Event) => {
		const detail = (e as Event & { detail: Date }).detail;
		const time = [
			detail.getHours().toString().padStart(2, '0'),
			detail.getMinutes().toString().padStart(2, '0')
		].join(':');
		if (!value.includes(time)) {
			value = [...value, time];
		}
		open = false;
	};
</script>

<div
	class="fixed w-screen h-screen bg-black bg-opacity-30 z-20 top-0 left-0 flex justify-center items-center"
	class:hidden={!open}
>
	<div class="relative">
		<button class="absolute top-0 right-2 z-20 text-white text-xl"
			><Icons name={Icon.Times} on:click={() => (open = false)} /></button
		>
		<TimePicker
			options={{
				clockClassName: 'clock',
				timeClassName: 'time',
				bgColor: '#636cce',
				hasButtons: true,
				buttonOk: 'Ok',
				buttonClassName: 'btn',
				buttonBarClassName: 'btn-bar',
				is24h: true
			}}
			on:ok={save}
		/>
	</div>
</div>

<div class="mb-6 relative">
	<Label>
		{label}
	</Label>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div
		on:click={() => {
			if (disabled) return;
			open = true;
		}}
		tabindex="-1"
		role="listbox"
		class={twMerge(modalSelectClass, sizes[size], $$props.class)}
	>
		<span class="flex gap-2 flex-wrap">
			{#if value.length}
				{#each value as item}
					<slot {item} clear={() => (value = [])}>
						<Badge
							color="primary"
							large={size === 'lg'}
							dismissable
							params={{ duration: 100 }}
							on:close={() => {
								value = value.filter((v) => v !== item);
							}}
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
						value = [];
					}}
					color="none"
					class="p-0 focus:ring-gray-400"
				/>
			{/if}
			<div class="w-[1px] bg-gray-300 dark:bg-gray-600" />
		</div>
	</div>
</div>
