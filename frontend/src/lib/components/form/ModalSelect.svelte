<script lang="ts">
	import { Icon } from '$lib/const/icon';
	import {
		Label,
		Modal,
		type FormSizeType,
		type SelectOptionType,
		Badge,
		CloseButton,
		Checkbox
	} from 'flowbite-svelte';
	import { twMerge } from 'tailwind-merge';
	import Icons from '../ui/Icons.svelte';
	import Input from '../ui/Input.svelte';
	import { createEventDispatcher } from 'svelte';

	export let label: string = '';
	export let items: SelectOptionType<any>[] = [];
	export let value: number[] = [];
	export let size: FormSizeType = 'md';
	export let icon: Icon | undefined = undefined;
	export let disabled: boolean = false;

	const dispatch = createEventDispatcher();

	const modalSelectClass: string =
		'relative border border-gray-300 flex items-center rounded-lg gap-2 dark:border-gray-600 focus-within:ring-1 focus-within:border-primary-500 ring-primary-500 dark:focus-within:border-primary-500 dark:ring-primary-500';
	const sizes = {
		sm: 'px-2 py-1',
		md: 'px-3 py-2',
		lg: 'px-4 py-3'
	};

	const clearAll = (e?: MouseEvent) => {
		if (disabled) return;
		e?.stopPropagation();
		value = [];
	};

	const selectAll = (e?: MouseEvent) => {
		if (disabled) return;
		e?.stopPropagation();
		value = items.map((item) => item.value);
		dispatch(
			'selected',
			items.filter((i) => value.includes(i.value))
		);
	};

	const selectOption = (val: number) => {
		if (disabled) return;
		if (value.includes(val)) {
			value = value.filter((item) => item !== val);
		} else {
			value = [...value, val];
		}
		dispatch(
			'selected',
			items.filter((i) => value.includes(i.value))
		);
	};

	const removeOption = (val: number) => {
		if (disabled) return;
		value = value.filter((item) => item !== val);
		dispatch(
			'selected',
			items.filter((i) => value.includes(i.value))
		);
	};

	let open = false;
	let search = '';

	$: filteredItems =
		search == ''
			? items
			: items.filter((item) => (item.name as string).toLowerCase().includes(search.toLowerCase()));
</script>

<Modal bind:open title={label}>
	<div class="w-full p-2 flex flex-col md:flex-row gap-2">
		<div class="h-1/2 md:h-full md:w-1/2">
			<Input
				bind:value={search}
				class="max-w-1/2 md:max-w-1/3 rounded-r-full rounded-l-full"
				placeholder="Cari..."
				startItem
			>
				<Icons name={Icon.Search} slot="startItem" class="text-gray-600" />
			</Input>
			<div class="p-2 rounded-md border border-gray-400 mt-4">
				<div class="flex items-center justify-between gap-2 md:h-full">
					<h1 class="mb-4">Pilihan</h1>
					<Checkbox
						on:click={() => {
							if (value.length === items.length) {
								clearAll();
							} else {
								selectAll();
							}
						}}
						checked={value.length === items.length}>Semua</Checkbox
					>
				</div>
				{#each filteredItems as item (item.value)}
					<Checkbox
						class="mb-4 flex gap-2 items-center"
						checked={value.includes(item.value)}
						on:click={() => selectOption(item.value)}
					>
						{#if icon}
							<Icons name={icon} />
						{/if}
						<span>{item.name}</span>
					</Checkbox>
				{/each}
			</div>
		</div>
		<div class="h-1/2 md:h-full md:w-1/2">
			<div class="p-2 rounded-md border border-gray-400">
				<h1 class="mb-4 block">Di pilih</h1>
				<span class="flex gap-2 flex-wrap">
					{#if value.length}
						{#each items.filter((item) => value.includes(item.value)) as item (item.value)}
							<slot {item} clear={() => removeOption(item.value)}>
								<Badge
									color="primary"
									large={true}
									dismissable
									params={{ duration: 100 }}
									on:close={() => removeOption(item.value)}
								>
									{item.name}
								</Badge>
							</slot>
						{/each}
					{/if}
				</span>
			</div>
		</div>
	</div>
</Modal>

<div class="mb-6">
	<Label>
		{label}
		<select {...$$restProps} bind:value hidden multiple on:change on:input {disabled}>
			{#each items as { value, name } (value)}
				<option {value}>{name}</option>
			{/each}
		</select>
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<div
			on:click={() => {
				if (disabled) return;
				open = !open;
			}}
			tabindex="-1"
			role="listbox"
			class={twMerge(modalSelectClass, sizes[size], $$props.class)}
		>
			<span class="flex gap-2 flex-wrap">
				{#if value.length}
					{#each items.filter((item) => value.includes(item.value)) as item (item.value)}
						<slot {item} clear={() => removeOption(item.value)}>
							<Badge
								color="primary"
								large={size === 'lg'}
								dismissable
								params={{ duration: 100 }}
								on:close={() => removeOption(item.value)}
							>
								{item.name}
							</Badge>
						</slot>
					{/each}
				{/if}
			</span>
			<div class="flex ml-auto gap-2 items-center">
				{#if value.length}
					<CloseButton on:click={clearAll} color="none" class="p-0 focus:ring-gray-400" />
				{/if}
				<div class="w-[1px] bg-gray-300 dark:bg-gray-600" />
				<svg
					class="cursor-pointer h-3 w-3 ml-1 text-gray-800 dark:text-white"
					aria-hidden="true"
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 10 6"
				>
					<path
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d={open ? 'm1 5 4-4 4 4' : 'm9 1-4 4-4-4'}
					/>
				</svg>
			</div>
		</div>
	</Label>
</div>
