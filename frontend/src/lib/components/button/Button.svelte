<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export { className as class };

	let className = '';

	const dispatch = createEventDispatcher();

	const onClick = (e: Event) => {
		e.stopPropagation();
		dispatch('click');
	};
</script>

<button class="{className} px-4 py-2 rounded" on:click={onClick}>
	<slot />
</button>
