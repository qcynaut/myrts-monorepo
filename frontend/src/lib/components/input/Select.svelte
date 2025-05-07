<script lang="ts">
	import type { NameValue } from '$lib/types/components';
	import Label from './Label.svelte';
	import AngleDown from '../icon/AngleDown.svelte';

	export let items: NameValue<number>[] = [];
	export let selected: number | undefined = undefined;
	export let label: string = '';
	export let placeholder: string = '';
	export let disabled: boolean = false;

	let open = false;
</script>

<Label>
	{label}
	<button
		class="block w-full text-start relative p-2 rounded bg-white border border-gray-200 {disabled
			? 'bg-gray-200 text-gray-500'
			: ''}"
		on:click={() => (open = !open)}
		{disabled}
	>
		{selected ? items.find((item) => item.value === selected)?.name : placeholder}
		<AngleDown class="w-4 h-4 absolute right-4 top-1/2 transform -translate-y-1/2" />
		<div class="options" class:open>
			{#each items as item}
				<button on:click={() => (selected = item.value)} class:selected={item.value === selected}
					>{item.name}</button
				>
			{/each}
		</div>
	</button>
</Label>

<style lang="scss">
	.options {
		@apply absolute top-full rounded left-0 w-full z-10 p-2 hidden bg-white shadow-md max-h-40 overflow-y-auto;

		&.open {
			@apply block;
		}

		button {
			@apply w-full rounded-md p-2 text-start;

			&:hover {
				@apply bg-primary-300 text-white;
			}

			&.selected {
				@apply bg-primary-600 text-white;
			}
		}
	}
</style>
