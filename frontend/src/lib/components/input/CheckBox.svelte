<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Check from '../icon/Check.svelte';

	export { className as class };
	export let checked: boolean = false;
	export let disabled: boolean = false;

	let className: string = '';

	const dispatch = createEventDispatcher();

	const onChange = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		dispatch('check', { checked: e.currentTarget.checked });
	};
</script>

<label class="flex items-center gap-2 cursor-pointer {className}">
	<input type="checkbox" class="hidden" bind:checked on:change={onChange} {disabled} />
	<div
		class="box w-4 h-4 flex items-center justify-center rounded border border-gray-400"
		class:checked
	>
		<Check class="icon fill-white w-3 h-3" />
	</div>
	<slot />
</label>

<style lang="scss">
	.box {
		.icon {
			@apply hidden;
		}
		&.checked {
			@apply bg-primary-600 border-none;

			.icon {
				@apply block;
			}
		}
	}
</style>
