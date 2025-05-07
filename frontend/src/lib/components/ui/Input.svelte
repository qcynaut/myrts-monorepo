<script lang="ts">
	export let placeholder: string | undefined = undefined;
	export let type: 'text' | 'email' | 'password' | string = 'text';
	export let endItem: boolean = false;
	export let startItem: boolean = false;
	let className: string = '';
	export { className as class };
	export let id: string | undefined = undefined;
	export let value: string | undefined = undefined;
	export let error: string | undefined = undefined;
	export let validate: (value: string) => boolean = () => true;
	export let valid: boolean = true;

	const onInput = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		value = (e.target as HTMLInputElement).value;
		if (validate) {
			valid = validate(value);
		}
	};
</script>

<div class="relative">
	{#if startItem}
		<div class="absolute left-3 top-1/2 transform -translate-y-1/2">
			<slot name="startItem" />
		</div>
	{/if}
	<input
		{id}
		{placeholder}
		{type}
		class="w-full py-2 px-3 {startItem
			? 'pl-8'
			: ''} border border-gray-300 bg-white rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500 {className}"
		class:error={!valid}
		{value}
		on:input={onInput}
	/>
	{#if endItem}
		<div class="absolute right-3 top-1/2 transform -translate-y-1/2">
			<slot name="endItem" />
		</div>
	{/if}
</div>
{#if error && !valid}
	<p class="mt-1 text-sm text-red-600">{error}</p>
{/if}

<style lang="scss">
	.error {
		@apply border-red-500 focus:ring-red-500 focus:border-red-500;
	}
</style>
