<script lang="ts">
	import { Icon } from '$lib/const/icon';
	import { uiStore } from '$lib/stores/ui';
	import Icons from '../ui/Icons.svelte';

	export let expanded: boolean = false;
	let className: string = '';
	export { className as class };

	$: sidebarExpanded = $uiStore.sidebarExpanded;
</script>

<div class="relative sidebar-dropdown {className}" class:expanded>
	<button
		class="w-full {sidebarExpanded ? 'md:flex' : 'flex md:block'} items-center gap-2"
		on:click={() => (expanded = !expanded)}
	>
		<slot />
		<Icons
			name={Icon.AngleUp}
			class="transform transition-all {!expanded ? 'rotate-180' : ''} {!sidebarExpanded
				? 'md:hidden'
				: ''}"
		/>
	</button>
	<div class="inner {sidebarExpanded ? 'pl-4 pt-2' : 'md:p-2 not-expanded'}">
		<slot name="inner" />
	</div>
</div>

<style lang="scss">
	.sidebar-dropdown {
		.inner {
			@apply hidden;
		}

		&.expanded .inner {
			@apply block;

			&.not-expanded {
				@apply block p-2 bg-primary-500 rounded-lg mt-4;
			}

			@media (max-width: 768px) {
				@apply pl-4 pt-2;
			}
		}
	}
</style>
