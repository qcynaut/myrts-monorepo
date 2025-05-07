<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import Times from '../icon/Times.svelte';
	export let open: boolean = false;
	export let closable: boolean = true;
	export { className as class };

	const dispatch = createEventDispatcher();

	let className = '';

	const close = () => {
		if (closable) {
			open = false;
			dispatch('close');
		}
	};
</script>

<div
	class="fixed w-screen h-screen top-0 left-0 flex justify-center items-center z-50 {!open
		? 'hidden'
		: ''}"
>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="absolute top-0 left-o bg-black bg-opacity-50 w-full h-full" on:click={close} />
	<div class="relative bg-white rounded-md {className}">
		{#if closable}
			<button class="absolute top-2 right-2" on:click={close}>
				<Times class="fill-gray-500 w-6 h-6" />
			</button>
		{/if}
		<slot />
	</div>
</div>
