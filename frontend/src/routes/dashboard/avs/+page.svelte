<script lang="ts">
	import { acceptAvs, deleteAvs, getAvs, updatePartialAvs } from '$lib/service/avs';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import type { Avs } from '$lib/types/response';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import { onMount } from 'svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import {
		Table,
		TableBodyCell,
		TableHead,
		TableHeadCell,
		TableBodyRow,
		TableBody
	} from 'flowbite-svelte';
	import Check from '$lib/components/icon/Check.svelte';
	import Pencil from '$lib/components/icon/Pencil.svelte';
	import Trash from '$lib/components/icon/Trash.svelte';
	import Eye from '$lib/components/icon/Eye.svelte';
	import Input from '$lib/components/input/Input.svelte';
	import Search from '$lib/components/icon/Search.svelte';
	import Modal from '$lib/components/modal/Modal.svelte';
	import Times from '$lib/components/icon/Times.svelte';
	import Tabs from '$lib/components/tabs/Tabs.svelte';
	import type { NameValue } from '$lib/types/components';
	import Label from '$lib/components/input/Label.svelte';
	import Button from '$lib/components/button/Button.svelte';
	import { notify } from '$lib/stores/toast';

	let avs: Avs[] = [];
	let detail: Avs | null = null;
	let edit: Avs | null = null;
	let del: Avs | null = null;
	let partialEdit: { address: string; description: string } = { address: '', description: '' };
	let search = '';

	const load = async () => {
		const token = $authStore.token ?? '';
		setLoading(true);
		const res = await getAvs(token);
		setLoading(false);
		if (res.error) {
			return toastError(res.error.error);
		}
		avs = res.result ?? [];
	};

	const accept = async (id: number) => {
		setLoading(true);
		const token = $authStore.token ?? '';
		const res = await acceptAvs(token, id);
		setLoading(false);
		if (res.error) {
			return toastError(res.error.error);
		}
		toastSuccess('Avs diterima');
		load();
	};

	type Network = {
		ip: string;
		netmask: string;
	};

	type Networks = {
		wlan0?: Network[];
		end0?: Network[];
		eth0?: Network[];
	};

	const decode_networks = (av: Avs) => {
		const networks: Networks = JSON.parse(av.networks ?? '{}');
		return networks;
	};

	const byteToGB = (bytes: number) => {
		return (bytes / 1024 / 1024 / 1024).toFixed(2);
	};

	const editTabs: NameValue<number>[] = [
		{
			name: 'IP',
			value: 1
		},
		{
			name: 'Data AVS',
			value: 2
		}
	];
	let selectedTab: number = 1;

	const updatePartial = async (id: number) => {
		const token = $authStore.token ?? '';
		setLoading(true);
		const res = await updatePartialAvs(token, id, partialEdit.address, partialEdit.description);
		setLoading(false);
		if (res.error || !res.result) {
			return notify(res.error?.error ?? 'Gagal mengupdate AVS', 'error');
		}
		notify('AVS updated', 'success');
		edit = null;
		if (res.result) {
			avs = avs.map((av) => {
				if (av.id == id) {
					av.address = res.result!.address;
					av.description = res.result!.description;
				}
				return av;
			});
		}
	};

	const delAvs = async (id: number) => {
		const token = $authStore.token ?? '';
		setLoading(true);
		const res = await deleteAvs(token, id);
		setLoading(false);
		if (res.error || !res.result) {
			return notify(res.error?.error ?? 'Gagal menghapus AVS', 'error');
		}
		notify('AVS deleted', 'success');
		del = null;
		avs = avs.filter((av) => av.id != id);
	};

	const maxChar = (str: string, max: number) => {
		if (str.length > max) {
			return str.substring(0, max) + '...';
		}
		return str;
	};

	onMount(() => {
		load();
	});

	$: user = $authStore.user;
	$: filtered =
		search == ''
			? avs
			: avs.filter(
					(av) =>
						av.address?.includes(search) ||
						av.description?.includes(search) ||
						av.unique_id?.includes(search)
			  );
</script>

<Modal open={del != null} closable={false} class="w-3/4 md:w-1/3">
	<div class="p-4 overflow-hidden">
		{#if del}
			<h1 class="text-xl font-semibold text-center">Peringatan</h1>
			<p class="mt-8 text-gray-500 text-center">Apakah anda yakin ingin menghapus AVS ini?</p>
			<div class="flex justify-end gap-2 mt-4 flex-1 items-end">
				<Button class="text-primary-600 border border-primary-600" on:click={() => (del = null)}
					>Batalkan</Button
				>
				<Button
					class="bg-red-600 text-white"
					on:click={() => {
						if (del) {
							delAvs(del.id);
						}
					}}>Hapus</Button
				>
			</div>
		{/if}
	</div>
</Modal>

<Modal
	open={edit != null}
	closable={false}
	class="w-3/4 md:w-1/3 h-2/3 overflow-hidden flex flex-col"
>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span class="font-semibold">Detail AVS</span>
		<button class="p-2 rounded-full bg-white bg-opacity-25" on:click={() => (edit = null)}>
			<Times class="w-4 h-4 fill-white" />
		</button>
	</div>
	<div class="flex-1 p-4 overflow-hidden flex flex-col">
		<Tabs items={editTabs} bind:active={selectedTab} containerClass="bg-gray-200" full />
		{#if selectedTab == 1}
			<div
				class="px-8 text-xs text-gray-400 text-center w-full h-full flex justify-center items-center"
			>
				<h1>Untuk sekarang IP Address hanya bisa di setting saat instalasi demi keamanan.</h1>
			</div>
		{:else}
			<div class="flex-1 px-2 overflow-hidden pt-8">
				{#if edit}
					<div class="h-full overflow-y-auto">
						<Label>
							Alamat
							<Input bind:value={partialEdit.address} />
						</Label>
						<Label>
							Deskripsi
							<Input bind:value={partialEdit.description} type="multiline" />
						</Label>
						<div class="mt-8 flex justify-end">
							<Button
								class="bg-primary-600 text-white"
								on:click={() => {
									if (edit) {
										updatePartial(edit.id);
									}
								}}>Simpan</Button
							>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</Modal>

<Modal
	open={detail != null}
	closable={false}
	class="w-3/4 md:w-1/3 h-2/3 overflow-hidden flex flex-col"
>
	<div class="p-4 rounded-t-md bg-primary-600 text-white flex justify-between items-center">
		<span class="font-semibold">Detail AVS</span>
		<button class="p-2 rounded-full bg-white bg-opacity-25" on:click={() => (detail = null)}>
			<Times class="w-4 h-4 fill-white" />
		</button>
	</div>
	<div class="flex-1 p-4 overflow-hidden">
		<div class="h-full w-full overflow-y-auto">
			{#if detail}
				<h1 class="font-semibold">Unique ID</h1>
				<div class="p-2 border border-gray-200 rounded-md mb-2">{detail.unique_id}</div>
				{#if detail.address}
					<h1 class="font-semibold">Alamat</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">{detail.address}</div>
				{/if}
				{#if detail.description}
					<h1 class="font-semibold">Deskripsi</h1>
					<p class="p-2 border border-gray-200 rounded-md mb-2">{detail.description}</p>
				{/if}
				{#if detail.disk_total && detail.disk_free}
					<h1 class="font-semibold">Penyimpanan</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">
						{byteToGB(Number(detail.disk_total) - Number(detail.disk_free))}GB / {byteToGB(
							Number(detail.disk_total)
						)} GB
					</div>
				{/if}
				{#if detail.mem_total && detail.mem_free}
					<h1 class="font-semibold">RAM</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">
						{byteToGB(Number(detail.mem_total) - Number(detail.mem_free))}GB / {byteToGB(
							Number(detail.mem_total)
						)} GB
					</div>
				{/if}
				{#if detail.cpu_temp}
					<h1 class="font-semibold">Suhu CPU</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">{detail.cpu_temp}°C</div>
				{/if}
				{#if detail.networks}
					{@const networks = decode_networks(detail)}
					<h1 class="font-semibold">IP</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">
						{networks.wlan0?.[0]?.ip ?? networks.eth0?.[0]?.ip ?? networks.end0?.[0]?.ip ?? '-'}
					</div>
					<h1 class="font-semibold">Subnet Mask</h1>
					<div class="p-2 border border-gray-200 rounded-md mb-2">
						{networks.wlan0?.[0]?.netmask ??
							networks.eth0?.[0]?.netmask ??
							networks.end0?.[0]?.netmask ??
							'-'}
					</div>
				{/if}
			{/if}
		</div>
	</div>
</Modal>

<DashboardInner title="AVS" subtitle="Daftar AVS">
	<div class="w-full flex-1 bg-white p-4 overflow-hidden">
		<div class="h-full w-full overflow-y-auto">
			<div class="mb-4">
				<Input placeholder="Cari AVS" bind:value={search}>
					<Search class="fill-gray-400 w-6 h-6" slot="startItem" />
				</Input>
			</div>
			<Table shadow>
				<TableHead class="text-xs bg-gray-200 rounded-t-md">
					<TableHeadCell>No</TableHeadCell>
					<TableHeadCell>Unique ID</TableHeadCell>
					<TableHeadCell>Alamat</TableHeadCell>
					<TableHeadCell>Deskripsi</TableHeadCell>
					<TableHeadCell>IP</TableHeadCell>
					<TableHeadCell>Subnet Mask</TableHeadCell>
					<TableHeadCell>Suhu</TableHeadCell>
					<TableHeadCell>Status</TableHeadCell>
					<TableHeadCell>Aksi</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each filtered as av, i}
						{@const networks = decode_networks(av)}
						{@const status = av.pending !== 0 ? 3 : av.status}
						<TableBodyRow>
							<TableBodyCell>{i + 1}</TableBodyCell>
							<TableBodyCell>{av.unique_id}</TableBodyCell>
							<TableBodyCell>{maxChar(av.address ?? '-', 15)}</TableBodyCell>
							<TableBodyCell>{maxChar(av.description ?? '-', 15)}</TableBodyCell>
							<TableBodyCell
								>{networks.wlan0?.[0]?.ip ??
									networks.eth0?.[0]?.ip ??
									networks.end0?.[0]?.ip ??
									'-'}</TableBodyCell
							>
							<TableBodyCell
								>{networks.wlan0?.[0]?.netmask ??
									networks.eth0?.[0]?.netmask ??
									networks.end0?.[0]?.netmask ??
									'-'}</TableBodyCell
							>
							<TableBodyCell>{av.cpu_temp ?? '-'}°C</TableBodyCell>
							<TableBodyCell>
								<span
									class="px-2 py-1 rounded md text-xs {status == 3
										? 'bg-yellow-200 text-yellow-600'
										: status == 1
										? 'bg-green-200 text-green-600'
										: 'bg-red-200 text-red-600'}"
									>{status == 3 ? 'Pending' : status == 1 ? 'Aktif' : 'Tidak Aktif'}</span
								>
							</TableBodyCell>
							<TableBodyCell class="flex items-center gap-2">
								{#if status == 3}
									<button on:click={() => accept(av.id)}>
										<Check class="w-4 h-4 fill-gray-500 hover:fill-primary-600" />
									</button>
								{/if}
								<button on:click={() => (detail = av)}>
									<Eye class="w-4 h-4 fill-gray-500 hover:fill-primary-600" />
								</button>
								{#if user && (user.role?.id == 1 || user.role?.id == 2)}
									<button
										on:click={() => {
											partialEdit.address = av.address ?? '';
											partialEdit.description = av.description ?? '';
											edit = av;
										}}
									>
										<Pencil class="w-4 h-4 fill-gray-500 hover:fill-primary-600" />
									</button>
									<button on:click={() => (del = av)}>
										<Trash class="w-4 h-4 fill-gray-500 hover:fill-primary-600" />
									</button>
								{/if}
							</TableBodyCell>
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	</div>
</DashboardInner>
