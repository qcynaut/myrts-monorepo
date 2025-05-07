<script lang="ts">
	import Input from './Input.svelte';
	import Search from '../icon/Search.svelte';

	export let value: string = '';
	export let placeholder: string = '';
	export let rounded: 'sm' | 'md' | 'lg' | 'xl' | 'full' = 'md';
</script>

<Input {rounded} type="text" {placeholder} bind:value>
	<Search class="fill-gray-400 w-6 h-6" slot="startItem" />
</Input>
