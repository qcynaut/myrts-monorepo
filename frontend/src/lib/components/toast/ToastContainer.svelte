<script lang="ts">
	import { toastStore } from '$lib/stores/toast';
	import { fade, fly } from 'svelte/transition';
	import Toast from './Toast.svelte';
	import { flip } from 'svelte/animate';

	$: toasts = $toastStore.toasts;
</script>

<ul>
	{#each toasts as toast (toast.id)}
		<li in:fly={{ y: 100, duration: 100 }} out:fade animate:flip={{ duration: 100 }}>
			<Toast item={toast} />
		</li>
	{/each}
</ul>

<style lang="scss">
	ul {
		@apply fixed top-0 right-0 z-50 flex flex-col gap-2 m-4 w-2/3 md:w-1/4;
	}

	li {
		@apply w-full;
	}
</style>
