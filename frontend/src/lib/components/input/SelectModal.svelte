<script lang="ts">
	import type { NameValue } from '$lib/types/components';
	import BadgeLabel from '../badge/BadgeLabel.svelte';
	import AngleDown from '../icon/AngleDown.svelte';
	import Search from '../icon/Search.svelte';
	import Server from '../icon/Server.svelte';
	import Times from '../icon/Times.svelte';
	import Modal from '../modal/Modal.svelte';
	import CheckBox from './CheckBox.svelte';
	import Input from './Input.svelte';
	import Label from './Label.svelte';

	export let placeholder = '';
	export let title = '';
	export let searchText = '';
	export let items: NameValue<number>[] = [];
	export let selected: NameValue<number>[] = [];
	export let disabled: boolean = false;

	let open = false;
	let search = '';

	const selectAll = () => {
		if (selected.length === items.length) {
			selected = [];
		} else {
			selected = items;
		}
	};

	const select = (item: NameValue<number>) => {
		if (selected.includes(item)) {
			selected = selected.filter((i) => i !== item);
		} else {
			selected = [...selected, item];
		}
	};

	const remove = (item: NameValue<number>) => {
		selected = selected.filter((i) => i !== item);
	};

	const removeAll = () => {
		selected = [];
	};

	$: filtered =
		search == '' ? items : items.filter((i) => i.name.toLowerCase().includes(search.toLowerCase()));
</script>

<Label>
	{title}
	<button
		class="relative w-full p-2 border border-gray-200 cursor-pointer flex flex-wrap items-center justify-start gap-2"
		on:click={() => (open = true)}
		{disabled}
	>
		{#if selected.length == 0}
			<span class="text-gray-500">{placeholder}</span>
		{:else}
			{#each selected as item}
				<BadgeLabel
					name={item.name}
					bg="primary-400"
					rounded="md"
					on:close={() => {
						if (!disabled) remove(item);
					}}
				/>
			{/each}
		{/if}
		<AngleDown class="w-4 h-4 fill-gray-500 absolute right-2 top-1/2 transform -translate-y-1/2" />
	</button>
</Label>

<Modal bind:open class="w-3/4 md:w-2/3 h-2/3 overflow-hidden flex flex-col" closable={false}>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span>{title}</span>
		<button class="p-2 rounded-full bg-white bg-opacity-25" on:click={() => (open = false)}>
			<Times class="w-4 h-4 fill-white" />
		</button>
	</div>
	<div class="flex-1 w-full h-full overflow-hidden">
		<div class="p-2 w-full h-full flex flex-col md:flex-row gap-2">
			<div class="w-full md:w-1/2 p-2 rounded-md border border-gray-200 h-full overflow-y-auto">
				<Input placeholder={searchText} bind:value={search}>
					<Search class="fill-gray-400 w-6 h-6" slot="endItem" />
				</Input>
				<div class="mt-2">
					<CheckBox
						class="mb-2"
						on:check={() => selectAll()}
						checked={selected.length === items.length}>Pilih semua</CheckBox
					>
					{#each filtered as item}
						<CheckBox class="mb-2" checked={selected.includes(item)} on:check={() => select(item)}>
							<div class="flex items-center gap-2 fill-primary-600">
								<Server class="w-4 h-4" />
								<span>{item.name}</span>
							</div>
						</CheckBox>
					{/each}
				</div>
			</div>
			<div class="w-full md:w-1/2 p-2 rounded-md border border-gray-200 h-full overflow-y-auto">
				<div class="flex justify-between">
					<h1 class="font-semibold">{selected.length} Dipilih</h1>
					<button class="text-primary-600" on:click={() => removeAll()}>hapus semua</button>
				</div>
				<div class="mt-4 flex flex-wrap gap-2">
					{#each selected as item}
						<BadgeLabel
							name={item.name}
							bg="primary-400"
							size="sm"
							rounded="full"
							on:close={() => remove(item)}
						/>
					{/each}
				</div>
			</div>
		</div>
	</div>
</Modal>
