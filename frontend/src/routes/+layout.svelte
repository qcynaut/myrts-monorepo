<script>
	import '../styles/app.css';
	import { pwaInfo } from 'virtual:pwa-info';
	import { uiStore } from '$lib/stores/ui';
	import Spinner from '$lib/components/ui/Spinner.svelte';
	import ToastContainer from '$lib/components/toast/ToastContainer.svelte';
	import { dev } from '$app/environment';
	import { inject } from '@vercel/analytics';

	inject({ mode: dev ? 'development' : 'production' });

	$: loading = $uiStore.loading;
	$: webManifestLink = pwaInfo ? pwaInfo.webManifest.linkTag : '';
</script>

<svelte:head>
	{@html webManifestLink}
</svelte:head>

<slot />

<ToastContainer />

{#if loading}
	<div
		class="fixed top-0 left-0 w-full h-full bg-black bg-opacity-50 flex items-center justify-center z-50"
	>
		<Spinner />
	</div>
{/if}
