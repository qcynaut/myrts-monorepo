<script lang="ts">
	import type { Icon } from '$lib/const/icon';
	import { createEventDispatcher } from 'svelte';

	export let name: Icon;
	let className: string = '';
	export { className as class };

	const dispatch = createEventDispatcher();

	const onClick = () => {
		dispatch('click');
	};
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<i class="{name} {className}" on:click={onClick} />
