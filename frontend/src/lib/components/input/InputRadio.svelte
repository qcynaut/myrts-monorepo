<script lang="ts">
	import type { NameValue } from '$lib/types/components';
	import Label from './Label.svelte';

	export let items: NameValue<number>[] = [];
	export let selected: number | undefined = undefined;
	export let disabled: boolean = false;

	let group = Math.random().toString(36).substring(2, 15);
</script>

<div class="flex gap-4 items-center">
	{#each items as item}
		<Label
			class="flex gap-2 items-center {disabled && selected != item.value ? 'text-gray-500' : ''}"
		>
			<input type="radio" bind:group={selected} name={group} value={item.value} {disabled} />
			<div class="radio" />
			{item.name}
		</Label>
	{/each}
</div>

<style lang="scss">
	input[type='radio'] {
		@apply hidden;
	}

	.radio {
		@apply w-4 h-4 flex items-center justify-center rounded-full border border-gray-400;
	}

	input[type='radio']:checked + .radio {
		@apply border-primary-500;

		&:after {
			content: '';
			@apply w-2 h-2 rounded-full bg-primary-500;
		}
	}

	input[type='radio']:disabled + :not(input[type='radio']:checked) + .radio {
		@apply border-gray-500;
	}
</style>
