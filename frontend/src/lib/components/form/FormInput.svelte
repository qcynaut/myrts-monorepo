<script lang="ts">
	import { Input, Label, type InputType, Helper, Textarea } from 'flowbite-svelte';

	export let label: string = '';
	export let type: InputType = 'text';
	export let placeholder: string = '';
	export let helper: string = '';
	export let valid: boolean = true;
	export let validate: (value: string) => boolean = () => true;
	export let value: string | undefined = undefined;
	export let disabled = false;
	export let multiline = false;

	const onInput = (e: Event) => {
		value = (e.target as HTMLInputElement).value;
		valid = validate(value);
	};
</script>

<div class="mb-6">
	<Label color={valid ? 'gray' : 'red'}>
		{label}
		{#if multiline}
			<Textarea {placeholder} on:change={onInput} {value} {disabled} />
		{:else}
			<Input
				{type}
				{placeholder}
				color={valid ? 'base' : 'red'}
				class="mt-2"
				on:change={onInput}
				{value}
				{disabled}
			/>
		{/if}
		{#if !valid}
			<Helper color="red">{helper}</Helper>
		{/if}
	</Label>
</div>
