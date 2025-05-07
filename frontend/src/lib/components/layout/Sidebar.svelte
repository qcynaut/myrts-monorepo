<script lang="ts">
	import { setLoading, setSidebarExpanded, uiStore } from '$lib/stores/ui';
	import { Avatar, Button, Modal } from 'flowbite-svelte';
	import IoTLogo from '../images/IoTLogo.svelte';
	import AvatarBlank from '../images/AvatarBlank.svelte';
	import SidebarGroup from './SidebarGroup.svelte';
	import SidebarItem from './SidebarItem.svelte';
	import { Icon } from '$lib/const/icon';
	import SidebarDropDown from './SidebarDropDown.svelte';
	import type { NavlinkType } from '$lib/types/navigation';
	import { DASHBOARD_PROFILE, navigations } from '$lib/const/navigation';
	import { authStore, setToken, setUser } from '$lib/stores/auth';
	import Icons from '../ui/Icons.svelte';
	import { QuestionCircleOutline } from 'flowbite-svelte-icons';
	import { toastError } from '$lib/utils/toast';
	import { logout } from '$lib/service/auth';
	import { get } from 'svelte/store';

	let popupModal = false;

	/// get navigation link.
	const getNavlink = () => {
		let navlink: Map<number, NavlinkType[]> = new Map();
		for (const [key, value] of Object.entries(navigations)) {
			if (value.link == DASHBOARD_PROFILE) continue;
			if (!navlink.has(value.group)) {
				navlink.set(value.group, [value]);
			} else {
				navlink.get(value.group)?.push(value);
			}
		}
		return navlink;
	};

	const signOut = () => {
		popupModal = true;
	};

	const confirmSignOut = async () => {
		setLoading(true);
		const res = await logout(get(authStore).token || '');
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		} else {
			localStorage.removeItem('token');
			setToken(null);
			setUser(null);
		}
	};

	$: expanded = $uiStore.sidebarExpanded;
	$: store = $authStore;
</script>

<Modal bind:open={popupModal} size="xs">
	<div class="text-center">
		<QuestionCircleOutline class="mx-auto mb-4 text-yellow-500 w-12 h-12" />
		<h3 class="mb-5 text-lg font-normal text-gray-500">Apakah anda yang ingin keluar?</h3>
		<Button
			color="red"
			class="mr-2"
			on:click={() => {
				popupModal = false;
				confirmSignOut();
			}}>Ya</Button
		>
		<Button
			color="primary"
			class="mr-2"
			on:click={() => {
				popupModal = false;
			}}>Tidak</Button
		>
	</div>
</Modal>

<div class="sidebar" class:expanded>
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div class="py-6 hidden md:block" on:click={() => setSidebarExpanded(!expanded)}>
		<IoTLogo variant={expanded ? 'large' : 'small'} />
	</div>
	<div class="md:hidden flex justify-end items-center p-4">
		<button on:click={() => setSidebarExpanded(!expanded)}>
			<Icons name={Icon.Times} />
		</button>
	</div>
	<a
		href={DASHBOARD_PROFILE}
		class="mt-3 mb-8 flex {expanded
			? 'jmd:ustify-start'
			: 'md:justify-center'} items-center gap-6 cursor-pointer"
	>
		{#if store.user?.image_url}
			<Avatar src={store.user.image_url} size="md" />
		{:else}
			<AvatarBlank width="35" height="35" />
		{/if}
		<div class:expanded id="info">
			<h1 class="font-bold">{store.user?.name ?? ''}</h1>
			<p class="text-xs">{store.user?.role?.name ?? ''}</p>
		</div>
	</a>
	{#each Array.from(getNavlink().entries()) as [key, value]}
		<SidebarGroup class="mb-4">
			{#each value as links}
				{#if links.kind == 'link'}
					<SidebarItem
						title={links.name}
						icon={links.icon ?? Icon.BorderAll}
						link={links.link}
						class={!links.user ? ($authStore.user?.role?.id == 3 ? 'hidden' : '') : ''}
						user={links.user}
					/>
				{:else}
					<SidebarDropDown>
						<SidebarItem title={links.name} icon={links.icon ?? Icon.BorderAll} />
						<svelte:fragment slot="inner">
							{#each Array.from(links.children ?? []) as link}
								<SidebarItem
									title={link.name}
									icon={link.icon ?? Icon.BorderAll}
									link={link.link}
									class="text-md mt-2"
									user={link.user}
								/>
							{/each}
						</svelte:fragment>
					</SidebarDropDown>
				{/if}
			{/each}
		</SidebarGroup>
	{/each}
	<SidebarItem
		title="Sign Out"
		icon={Icon.SignOutAlt}
		button={true}
		on:click={signOut}
		class="mt-auto ml-2 mb-8"
	/>
	<div class="flex justify-center text-xs text-gray-200">
		<span>MyRTS v1.0.0</span>
	</div>
</div>

<style lang="scss">
	.sidebar {
		@apply transition-all duration-300 h-screen overflow-y-auto bg-primary-600 p-4 w-20 rounded-r-xl text-white;
	}

	@media (min-width: 768px) {
		.sidebar {
			@apply relative block;

			#info {
				@apply hidden;
			}

			&.expanded {
				@apply w-2/12;

				#info {
					@apply block;
				}
			}
		}
	}

	@media (max-width: 768px) {
		.sidebar {
			@apply absolute w-[70%] h-screen top-0 z-20 rounded-none right-0;

			&.expanded {
				@apply -right-full;
			}
		}
	}
</style>
