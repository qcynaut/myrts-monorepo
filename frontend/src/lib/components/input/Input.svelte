<script lang="ts">
	import { twMerge } from 'tailwind-merge';

	export let value: string = '';
	export let placeholder: string = '';
	export let type: 'text' | 'number' | 'email' | 'password' | 'multiline' = 'text';
	export { className as class };
	export let rounded: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'md';
	export let disabled: boolean = false;

	let className = '';
	let focus = false;

	$: classes = twMerge(
		rounded == 'full' ? 'rounded-r-full rounded-l-full' : `rounded-${rounded}`,
		className
	);
</script>

<div class="relative z-0 w-full" class:focus>
	<slot name="startItem" />
	<slot name="endItem" />
	{#if type == 'number'}
		<input
			type="number"
			class={classes}
			{placeholder}
			bind:value
			on:focus={() => (focus = true)}
			on:blur={() => (focus = false)}
			{disabled}
		/>
	{:else if type == 'email'}
		<input
			type="email"
			class={classes}
			{placeholder}
			bind:value
			on:focus={() => (focus = true)}
			on:blur={() => (focus = false)}
			{disabled}
		/>
	{:else if type == 'password'}
		<input
			type="password"
			class={classes}
			{placeholder}
			bind:value
			on:focus={() => (focus = true)}
			on:blur={() => (focus = false)}
			{disabled}
		/>
	{:else if type == 'multiline'}
		<textarea
			class={classes}
			{placeholder}
			bind:value
			on:focus={() => (focus = true)}
			on:blur={() => (focus = false)}
			{disabled}
		/>
	{:else}
		<input
			type="text"
			class={classes}
			{placeholder}
			bind:value
			on:focus={() => (focus = true)}
			on:blur={() => (focus = false)}
			{disabled}
		/>
	{/if}
</div>

<style lang="scss">
	div {
		input,
		textarea {
			@apply w-full border-gray-200 focus:border-primary-600 focus:ring-0;
		}

		input:disabled,
		textarea:disabled {
			@apply bg-gray-100 text-gray-500;
		}

		:global([slot='endItem'] + input) {
			@apply pr-10;
		}
		:global([slot='endItem'] + textarea) {
			@apply pr-10;
		}

		:global([slot='startItem'] + input) {
			@apply pl-10;
		}
		:global([slot='startItem'] + textarea) {
			@apply pl-10;
		}

		:global([slot='startItem']) {
			@apply absolute left-3 top-1/2 transform -translate-y-1/2;
		}

		:global([slot='endItem']) {
			@apply absolute right-3 top-1/2 transform -translate-y-1/2;
		}

		&.focus {
			:global([slot='startItem']) {
				@apply fill-primary-600;
			}
		}
	}
</style>
