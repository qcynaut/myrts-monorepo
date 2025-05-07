<script lang="ts">
	import type { Icon } from '$lib/const/icon';
	import { uiStore } from '$lib/stores/ui';
	import { createEventDispatcher } from 'svelte';
	import Icons from '../ui/Icons.svelte';
	import { authStore } from '$lib/stores/auth';

	export let title: string;
	export let icon: Icon;
	export let link: string | undefined = undefined;
	let className: string = '';
	export { className as class };
	export let button: boolean = false;
	export let user = true;

	const dispatch = createEventDispatcher();

	const onClick = (e: Event) => {
		e.stopPropagation();
		dispatch('click');
	};

	$: expanded = $uiStore.sidebarExpanded;
	$: hidden = !user && $authStore.user?.role?.id == 3;
</script>

{#if button}
	<button
		on:click={onClick}
		class="s-group flex items-center justify-center cursor-pointer {className}"
		class:expanded
		class:hidden
	>
		<Icons name={icon} class="text-xl" />
		<h1 class="hidden">{title}</h1>
	</button>
{:else}
	<a
		href={link}
		class="s-group flex items-center justify-center {className} cursor-pointer"
		class:expanded
		class:hidden
	>
		<Icons name={icon} class="text-xl" />
		<h1 class="hidden">{title}</h1>
	</a>
{/if}

<style lang="scss">
	@media (min-width: 768px) {
		.expanded {
			@apply justify-start gap-4;
			h1 {
				@apply block;
			}
		}
	}

	@media (max-width: 768px) {
		.s-group {
			@apply justify-start gap-4;
			h1 {
				@apply block;
			}
		}
	}
</style>
