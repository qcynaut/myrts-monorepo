<script lang="ts">
	import InfoCircle from '../icon/InfoCircle.svelte';
	import CheckCircle from '../icon/CheckCircle.svelte';
	import TimesCircle from '../icon/TimesCircle.svelte';
	import ExclamationCircle from '../icon/ExclamationCircle.svelte';
	import { removeToast, type ToastItem } from '$lib/stores/toast';
	import { onMount } from 'svelte';

	export let item: ToastItem;

	onMount(() => {
		setTimeout(() => {
			removeToast(item);
		}, 3000);
	});
</script>

<div class="w-full p-2 rounded-md {item.type}">
	{#if item.type == 'info'}
		<InfoCircle />
	{:else if item.type == 'success'}
		<CheckCircle />
	{:else if item.type == 'warning'}
		<ExclamationCircle />
	{:else if item.type == 'error'}
		<TimesCircle />
	{/if}
	<p>{item.message}</p>
</div>

<style lang="scss">
	div {
		@apply flex items-center gap-2;

		:global(:first-child) {
			@apply w-6 h-6;
		}
	}

	.info {
		@apply bg-primary-300 text-primary-600;

		:global(:first-child) {
			@apply fill-primary-600;
		}
	}

	.success {
		@apply bg-green-300 text-green-600;

		:global(:first-child) {
			@apply fill-green-600;
		}
	}

	.warning {
		@apply bg-yellow-300 text-yellow-600;

		:global(:first-child) {
			@apply fill-yellow-600;
		}
	}

	.error {
		@apply bg-red-300 text-red-600;

		:global(:first-child) {
			@apply fill-red-600;
		}
	}
</style>
