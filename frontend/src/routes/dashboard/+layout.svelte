<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import IoTLogo from '$lib/components/images/IoTLogo.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import { Icon } from '$lib/const/icon';
	import { current } from '$lib/service/user';
	import { authStore, setToken, setUser } from '$lib/stores/auth';
	import { setSidebarExpanded, uiStore } from '$lib/stores/ui';
	import { get } from 'svelte/store';
	import { SvelteToast } from '@zerodevx/svelte-toast';

	const getUser = async (token: string) => {
		const res = await current(token);
		if (res.result) {
			setUser(res.result);
		} else if (res.status == 401) {
			localStorage.removeItem('token');
			setToken(null);
		}
	};

	authStore.subscribe((value) => {
		if (!value.token) {
			if (!browser) {
				return;
			}
			let localToken = localStorage.getItem('token');
			if (localToken) {
				setToken(localToken);
				return;
			} else {
				goto('/', { replaceState: true });
			}
		} else {
			if (!get(authStore).user) {
				getUser(value.token);
			}
		}
	});

	$: expanded = $uiStore.sidebarExpanded;
</script>

<SvelteToast />

<div class="relative h-screen w-screen flex flex-col md:flex-row overflow-hidden">
	<Sidebar />
	<nav class="navbar w-full p-4 md:hidden bg-white flex justify-between items-center">
		<IoTLogo class="w-2/5" />
		<button on:click={() => setSidebarExpanded(!expanded)}>
			<Icons name={Icon.Bars} class="text-2xl" />
		</button>
	</nav>
	<div class="relative w-full h-screen overflow-y-auto">
		<slot />
	</div>
</div>

<style lang="scss">
	.navbar {
		@apply relative z-10;
		box-shadow: 0 2px 8px -2px gray;
	}
</style>
