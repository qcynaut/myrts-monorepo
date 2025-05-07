<script lang="ts">
	import { validateEmail } from '$lib/utils/validation';
	import { Button, Helper, Input, Label, Select } from 'flowbite-svelte';
	import FormInput from './FormInput.svelte';
	import { onMount } from 'svelte';
	import { getProvinces } from '$lib/service/province';
	import { get } from 'svelte/store';
	import { authStore } from '$lib/stores/auth';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import { getCites } from '$lib/service/city';
	import { getPackages } from '$lib/service/package';
	import ModalSelect from './ModalSelect.svelte';
	import { getGroups } from '$lib/service/group';
	import { Icon } from '$lib/const/icon';
	import { getAvs } from '$lib/service/avs';
	import { createUser, updateUser } from '$lib/service/user';
	import { setLoading } from '$lib/stores/ui';
	import Icons from '../ui/Icons.svelte';
	import { goto } from '$app/navigation';
	import { DASHBOARD_USER } from '$lib/const/navigation';

	export let id: number | undefined = undefined;
	export let name: string = '';
	export let role: number = 3;
	export let email: string = '';
	export let devices: number[] = [];
	export let group_ids: number[] = [];
	export let city_id: number = 0;
	export let province_id: number = 0;
	export let package_id: number = 0;
	export let kind: 'add' | 'edit' | 'view' = 'add';
	export let back: (reload: boolean) => void = () => {};

	let roles: { value: number; name: string }[] = [
		{ value: 3, name: 'Admin' },
		{ value: 2, name: 'SuperAdmin' },
		{ value: 1, name: 'Root' }
	];

	let provinces: { value: number; name: string }[] = [];
	let cities: { value: number; name: string; province_id: number }[] = [];
	let packages: { value: number; name: string }[] = [];
	let groups: { value: number; name: string }[] = [];
	let avs: { value: number; name: string }[] = [];

	let validName: boolean = true;
	let validEmail: boolean = true;

	const toNull = (num: number) => (num == 0 ? null : num);

	const submit = async () => {
		if (name === '') return (validName = false);
		if (!validateEmail(email)) return (validEmail = false);
		if (role === 3) {
			if (province_id === 0) return toastError('Pilih Provinsi');
			if (city_id === 0) return toastError('Pilih Kota');
			if (package_id === 0) return toastError('Pilih Paket');
		}
		setLoading(true);
		const res = await createUser(
			get(authStore).token || '',
			name,
			email,
			role,
			toNull(package_id),
			toNull(city_id),
			devices,
			group_ids
		);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
			return;
		} else {
			toastSuccess('Berhasil menambahkan user');
			goto(DASHBOARD_USER, { replaceState: true });
		}
	};

	const update = async () => {
		if (!id) return toastError('ID Tidak Ditemukan');
		if (name === '') return (validName = false);
		if (!validateEmail(email)) return (validEmail = false);
		if (role === 3) {
			if (province_id === 0) return toastError('Pilih Provinsi');
			if (city_id === 0) return toastError('Pilih Kota');
			if (package_id === 0) return toastError('Pilih Paket');
		}
		setLoading(true);
		const res = await updateUser(
			get(authStore).token || '',
			name,
			email,
			role,
			toNull(city_id),
			devices,
			group_ids,
			id
		);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
			return;
		} else {
			toastSuccess('Berhasil update user');
			back(true);
		}
	};

	onMount(async () => {
		setLoading(true);
		const token = get(authStore).token || '';
		const pres = await getProvinces(token);
		if (pres.error) {
			toastError(pres.error.error);
		}
		provinces =
			pres.result
				?.map((p) => {
					return {
						value: p.id,
						name: p.name
					};
				})
				.sort((a, b) => a.name.localeCompare(b.name)) || [];

		const cres = await getCites(token);
		if (cres.error) {
			toastError(cres.error.error);
		}
		cities =
			cres.result
				?.map((c) => {
					return {
						value: c.id,
						name: c.name,
						province_id: c.province_id
					};
				})
				.sort((a, b) => a.name.localeCompare(b.name)) || [];

		const pcres = await getPackages(token);
		if (pcres.error) {
			toastError(pcres.error.error);
		}
		packages =
			pcres.result?.map((c) => {
				return {
					value: c.id,
					name: c.name
				};
			}) || [];

		const gres = await getGroups(token);
		if (gres.error) {
			toastError(gres.error.error);
		}
		groups =
			gres.result?.map((c) => {
				return {
					value: c.id,
					name: c.name
				};
			}) || [];

		const ares = await getAvs(token);
		if (ares.error) {
			toastError(ares.error.error);
		}
		avs =
			ares.result?.map((c) => {
				return {
					value: c.id,
					name: c.address ? c.address : c.description ? c.description : c.unique_id
				};
			}) || [];

		setLoading(false);
	});
</script>

{#if kind != 'add'}
	<div class="my-6 flex justify-end">
		<button
			class="flex items-center gap-2 rounded-r-full rounded-l-full py-2 px-4 bg-primary-500 text-white"
			on:click={() => back(false)}
		>
			<Icons name={Icon.ArrowLeft} />
			<span>Kembali</span>
		</button>
	</div>
{/if}

<div class="mb-6">
	<Label>
		Role
		<Select
			class="mt-2"
			items={roles}
			bind:value={role}
			placeholder="Pilih Role"
			disabled={kind != 'add'}
		/>
	</Label>
</div>
<FormInput
	label="Nama"
	bind:value={name}
	placeholder="Masukan Nama"
	helper="Nama tidak boleh kosong"
	valid={validName}
	disabled={kind == 'view'}
/>
<FormInput
	label="Email"
	bind:value={email}
	type="email"
	validate={validateEmail}
	valid={validEmail}
	placeholder="Masukan Email"
	helper="Email tidak valid"
	disabled={kind == 'view'}
/>
{#if role == 3}
	<div class="mb-6">
		<Label>
			Provinsi
			<Select
				class="mt-2"
				items={provinces}
				bind:value={province_id}
				placeholder="Pilih Provinsi"
				disabled={kind == 'view'}
			/>
		</Label>
	</div>
	{#if province_id != 0}
		<div class="mb-6">
			<Label>
				Kabupaten/Kota
				<Select
					class="mt-2"
					items={cities.filter((c) => c.province_id == province_id)}
					bind:value={city_id}
					placeholder="Pilih Kabupaten/Kota"
					disabled={kind == 'view'}
				/>
			</Label>
		</div>
	{/if}
	<div class="mb-6">
		<Label>
			Paket
			<Select
				class="mt-2"
				items={packages}
				bind:value={package_id}
				placeholder="Pilih paket"
				disabled={kind != 'add'}
			/>
		</Label>
	</div>
	<ModalSelect
		label="AVS"
		items={avs}
		bind:value={devices}
		icon={Icon.RaspberryPi}
		disabled={kind == 'view'}
	/>
{/if}
<ModalSelect
	label="Groups"
	bind:items={groups}
	bind:value={group_ids}
	icon={Icon.Users}
	disabled={kind == 'view'}
/>

{#if kind != 'view'}
	<div class="my-6 flex justify-center">
		<Button
			on:click={() => {
				if (kind == 'add') {
					submit();
				} else if (kind == 'edit') {
					update();
				}
			}}>Submit</Button
		>
	</div>
{/if}
