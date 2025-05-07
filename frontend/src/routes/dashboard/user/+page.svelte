<script lang="ts">
	import GroupForm from '$lib/components/form/GroupForm.svelte';
	import UserForm from '$lib/components/form/UserForm.svelte';
	import Layout from '$lib/components/layout/Layout.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import TableAction from '$lib/components/ui/TableAction.svelte';
	import TableGroupBody from '$lib/components/ui/TableGroupBody.svelte';
	import { Icon } from '$lib/const/icon';
	import { get_group_paginated } from '$lib/service/group';
	import { deleteUser } from '$lib/service/user';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import type { GroupData, User } from '$lib/types/response';
	import { utcToLocal } from '$lib/utils/format';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import {
		Pagination,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		type LinkType
	} from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { utils, writeFileXLSX } from 'xlsx';

	let groups: { page: number; data: GroupData[] }[] = [];
	let totalPage = 0;
	let currentPage = 1;
	let current: GroupData[] = [];
	let search = '';
	let el: HTMLDivElement;
	let dataUser: User | null = null;
	let dataGroup: GroupData | null = null;

	const load = async () => {
		setLoading(true);
		const res = await get_group_paginated(currentPage, get(authStore).token || '');
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		} else {
			if (res.result?.items) {
				groups.push({ page: res.result.current_page, data: res.result.items });
				totalPage = res.result.total_page;
				currentPage = res.result.current_page;
				current = res.result?.items || [];
			}
		}
	};

	const prev = async () => {
		if (currentPage > 1) {
			let newCurrentPage = currentPage - 1;
			let groups_prev = groups.filter((g) => g.page == newCurrentPage);
			if (groups_prev.length > 0) {
				currentPage = newCurrentPage;
				current = groups_prev[0].data;
				return;
			}
			const res = await get_group_paginated(newCurrentPage, get(authStore).token || '');
			if (res.error) {
				toastError(res.error.error);
			} else {
				if (res.result?.items) {
					groups.push({ page: res.result.current_page, data: res.result.items });
					totalPage = res.result.total_page;
					currentPage = res.result.current_page;
					current = res.result?.items || [];
				}
			}
		}
	};

	const next = async () => {
		if (currentPage < totalPage) {
			let newCurrentPage = currentPage + 1;
			let groups_next = groups.filter((g) => g.page == newCurrentPage);
			if (groups_next.length > 0) {
				currentPage = newCurrentPage;
				current = groups_next[0].data;
				return;
			}
			const res = await get_group_paginated(newCurrentPage, get(authStore).token || '');
			if (res.error) {
				toastError(res.error.error);
			} else {
				if (res.result?.items) {
					groups.push({ page: res.result.current_page, data: res.result.items });
					totalPage = res.result.total_page;
					currentPage = res.result.current_page;
					current = res.result?.items || [];
				}
			}
		}
	};

	onMount(async () => {
		if (groups.length == 0) {
			await load();
		}
	});

	function nestedSearch(data: GroupData[], search: string): GroupData[] {
		if (search == '') return data;
		const res: GroupData[] = [];
		const lowercaseSearch = search.toLowerCase();

		for (const d of data) {
			if (d.name.toLowerCase().includes(lowercaseSearch)) {
				res.push(d);
			}

			for (const u of d.users) {
				if (u.name.toLowerCase().includes(lowercaseSearch)) {
					res.push(d);
					break;
				}
			}

			if (d.children.length > 0) {
				res.push(...nestedSearch(d.children, search));
			}
		}

		return res;
	}

	const exportUser = () => {
		const doc = el.getElementsByTagName('table')[0];
		const wb = utils.table_to_book(doc);
		writeFileXLSX(wb, 'users.xlsx');
	};

	const setEdit = (kind: 'user' | 'group', data: User | GroupData) => {
		formKind = 'edit';
		switch (kind) {
			case 'user':
				dataUser = data as User;
				break;
			case 'group':
				dataGroup = data as GroupData;
				break;
		}
		action = true;
		actionKind = kind;
	};

	const setView = (kind: 'user' | 'group', data: User | GroupData) => {
		formKind = 'view';
		switch (kind) {
			case 'user':
				dataUser = data as User;
				break;
			case 'group':
				dataGroup = data as GroupData;
				break;
		}
		action = true;
		actionKind = kind;
	};

	const back = (reload: boolean) => {
		action = false;
		actionKind = 'user';
		formKind = 'view';
		if (reload) {
			load();
		}
	};

	let action = false;
	let actionKind: 'user' | 'group' = 'user';
	let formKind: 'edit' | 'view' = 'view';

	const deleteUserHandle = async (id: number) => {
		setLoading(true);
		const res = await deleteUser(get(authStore).token ?? '', id);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		} else {
			toastSuccess('User deleted');
			load();
		}
	};

	$: filtered = nestedSearch(current, search);
	$: currentUser = $authStore.user;
	$: buildUserForm = () => {
		let form = {
			city_id: dataUser?.city?.id || 0,
			devices: dataUser?.device_ids || [],
			email: dataUser?.email || '',
			group_ids: dataUser?.user_group_ids || [],
			name: dataUser?.name || '',
			package_id: dataUser?.subscription?.package.id || 0,
			province_id: dataUser?.city?.province.id || 0,
			role: dataUser?.role?.id || 0,
			id: dataUser?.id || 0
		};
		return form;
	};
	$: buildGroupForm = () => {
		let form = {
			name: dataGroup?.name || '',
			description: dataGroup?.description || '',
			parent_id: dataGroup?.parent_id || null,
			id: dataGroup?.id || 0
		};
		return form;
	};
	$: userForm = buildUserForm();
	$: groupForm = buildGroupForm();
</script>

{#if action}
	<Layout
		title={actionKind == 'user' ? 'User' : 'Group'}
		subtitle={formKind == 'edit'
			? actionKind == 'user'
				? 'Edit User'
				: 'Edit Group'
			: actionKind == 'user'
			? 'Detail User'
			: 'Detail Group'}
	>
		<div class="mt-4">
			{#if actionKind == 'user'}
				<UserForm
					city_id={userForm.city_id}
					devices={userForm.devices}
					email={userForm.email}
					group_ids={userForm.group_ids}
					name={userForm.name}
					package_id={userForm.package_id}
					province_id={userForm.province_id}
					role={userForm.role}
					kind={formKind}
					id={userForm.id}
					{back}
				/>
			{:else}
				<GroupForm
					id={groupForm.id}
					name={groupForm.name}
					description={groupForm.description}
					parent_id={groupForm.parent_id}
					kind={formKind}
					{back}
				/>
			{/if}
		</div>
	</Layout>
{:else}
	<Layout title="User" subtitle="Manage User">
		<div class="mt-4 flex justify-between gap-8 items-center">
			<button
				class="flex items-center gap-2 text-white py-2 px-4 bg-primary-500 rounded-l-full rounded-r-full"
				on:click={exportUser}><span>export</span><Icons name={Icon.AngleDown} /></button
			>
			<Input
				bind:value={search}
				class="max-w-1/2 md:max-w-1/3 rounded-r-full rounded-l-full"
				placeholder="Cari..."
				startItem
			>
				<Icons name={Icon.Search} slot="startItem" class="text-gray-600" />
			</Input>
		</div>
		<div class="mt-4" bind:this={el}>
			<Table shadow={true}>
				<TableHead class="text-xs">
					<TableHeadCell />
					<TableHeadCell>Nama</TableHeadCell>
					<TableHeadCell>Provinsi</TableHeadCell>
					<TableHeadCell>Kabupaten/Kota</TableHeadCell>
					<TableHeadCell>Paket</TableHeadCell>
					<TableHeadCell>Order Date</TableHeadCell>
					<TableHeadCell>Role</TableHeadCell>
					<TableHeadCell>Keterangan</TableHeadCell>
					<TableHeadCell>Actions</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each filtered as data}
						{#if data.name == '_'}
							{#each data.users as user}
								<TableBodyRow
									class={currentUser?.id == user.id ? 'bg-primary-500 bg-opacity-25' : ''}
								>
									<TableBodyCell>
										<Icons name={Icon.User} />
									</TableBodyCell>
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
											remove={user.id != currentUser?.id
												? () => deleteUserHandle(user.id)
												: undefined}
										/>
									</TableBodyCell>
								</TableBodyRow>
							{/each}
						{:else}
							<TableGroupBody {data} {setEdit} {setView} removeUser={deleteUserHandle} />
						{/if}
					{/each}
				</TableBody>
			</Table>
			<div class="mt-6 flex justify-end">
				<Pagination on:previous={prev} on:next={next} />
			</div>
		</div>
	</Layout>
{/if}
