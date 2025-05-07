<script lang="ts">
	import type { GroupData, User } from '$lib/types/response';
	import { TableBodyCell, TableBodyRow } from 'flowbite-svelte';
	import Icons from './Icons.svelte';
	import { Icon } from '$lib/const/icon';
	import { utcToLocal } from '$lib/utils/format';
	import { get } from 'svelte/store';
	import { authStore } from '$lib/stores/auth';
	import TableAction from './TableAction.svelte';

	export let extraClass = '';
	export let nextClass = '';
	export let data: GroupData;
	export let setEdit: (kind: 'group' | 'user', data: User | GroupData) => void = () => {};
	export let setView: (kind: 'group' | 'user', data: User | GroupData) => void = () => {};
	export let removeUser: (id: number) => void = () => {};

	const colors = ['ctab-1', 'ctab-2', 'ctab-4', 'ctab-5'];

	const getNextClass = () => {
		if (nextClass == '') {
			// pick a random color
			return colors[Math.floor(Math.random() * colors.length)];
		}
		return nextClass;
	};

	const parsedNextClass = getNextClass();

	let expanded = false;
</script>

<TableBodyRow
	class="cursor-pointer {expanded ? parsedNextClass : extraClass}"
	on:click={() => (expanded = !expanded)}
>
	<TableBodyCell><Icons name={expanded ? Icon.AngleUp : Icon.AngleDown} /></TableBodyCell>
	<TableBodyCell>{data.name}</TableBodyCell>
	<TableBodyCell />
	<TableBodyCell />
	<TableBodyCell />
	<TableBodyCell class="whitespace-pre-line" colspan={3}>{data.description ?? ''}</TableBodyCell>
	<TableBodyCell>
		<TableAction view={() => setView('group', data)} edit={() => setEdit('group', data)} />
	</TableBodyCell>
</TableBodyRow>
{#if expanded}
	{#each data.users as user}
		<TableBodyRow class={parsedNextClass}>
			<TableBodyCell><Icons name={Icon.User} /></TableBodyCell>
			<TableBodyCell>{user.name}</TableBodyCell>
			<TableBodyCell>{user.city?.province.name ?? ''}</TableBodyCell>
			<TableBodyCell>{user.city?.name ?? ''}</TableBodyCell>
			<TableBodyCell>{user.subscription?.package.name ?? ''}</TableBodyCell>
			<TableBodyCell
				>{user.subscription?.order_date
					? utcToLocal(user.subscription.order_date)
					: ''}</TableBodyCell
			>
			<TableBodyCell
				class={user.role?.id == 1
					? 'text-primary-500'
					: user.role?.id == 2
					? 'text-green-600'
					: 'text-yellow-600'}>{user.role?.name ?? ''}</TableBodyCell
			>
			<TableBodyCell />
			<TableBodyCell>
				<TableAction
					view={() => setView('user', user)}
					edit={() => setEdit('user', user)}
					remove={user.id != get(authStore).user?.id ? () => removeUser(user.id) : undefined}
					removeConfirm="Apakah anda yakin ingin menghapus {user.name}?"
				/>
			</TableBodyCell>
		</TableBodyRow>
	{/each}
	{#each data.children as child}
		<svelte:self
			data={child}
			extraClass={parsedNextClass}
			nextClass={getNextClass()}
			{setEdit}
			{setView}
			{removeUser}
		/>
	{/each}
{/if}

<style lang="scss">
	:global(.ctab-1) {
		@apply bg-slate-200;
	}

	:global(.ctab-2) {
		@apply bg-red-300;
	}

	:global(.ctab-3) {
		@apply bg-green-300;
	}

	:global(.ctab-4) {
		@apply bg-purple-300;
	}

	:global(.ctab-5) {
		@apply bg-blue-300;
	}
</style>
