<script lang="ts">
	import { uiStore } from '$lib/stores/ui';

	let className: string = '';
	export { className as class };
	export let bordered: boolean = true;

	$: expanded = $uiStore.sidebarExpanded;
</script>

<div
	class="s-group-r relative px-2 py-4 bg-primary-700 rounded-xl flex flex-col gap-4 {className}"
	class:expanded
	class:bordered
>
	<slot />
</div>

<style lang="scss">
	@media (min-width: 768px) {
		.expanded {
			@apply bg-transparent rounded-none;

			&.bordered {
				@apply border-t  border-t-white;
			}
		}
	}

	@media (max-width: 768px) {
		.s-group-r {
			@apply bg-transparent rounded-none;

			&.bordered {
				@apply border-t  border-t-white;
			}
		}
	}
</style>
