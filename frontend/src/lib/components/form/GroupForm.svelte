<script lang="ts">
	import type { Group } from '$lib/types/response';
	import { onMount } from 'svelte';
	import FormInput from './FormInput.svelte';
	import { createGroup, getGroups, updateGroup } from '$lib/service/group';
	import { get } from 'svelte/store';
	import { authStore } from '$lib/stores/auth';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import { Button, Label, Select } from 'flowbite-svelte';
	import { setLoading } from '$lib/stores/ui';
	import { goto } from '$app/navigation';
	import { DASHBOARD_USER } from '$lib/const/navigation';
	import Icons from '../ui/Icons.svelte';
	import { Icon } from '$lib/const/icon';

	export let id: number | undefined = undefined;
	export let name: string = '';
	export let description: string = '';
	export let parent_id: number | null = null;
	export let kind: 'add' | 'edit' | 'view' = 'add';
	export let back: (reload: boolean) => void = () => {};

	let validName: boolean = true;

	let groups: { value: number; name: string }[] = [];

	const submit = async () => {
		if (name === '') return (validName = false);
		setLoading(true);
		const res = await createGroup(
			get(authStore).token ?? '',
			name,
			description === '' ? null : description,
			parent_id
		);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
			return;
		} else {
			toastSuccess('Group berhasil ditambahkan');
			goto(DASHBOARD_USER, { replaceState: true });
		}
	};

	const update = async () => {
		if (!id || id == 0) return toastError('ID Tidak Ditemukan');
		if (name === '') return (validName = false);
		setLoading(true);
		const res = await updateGroup(
			get(authStore).token ?? '',
			id,
			name,
			description === '' ? null : description,
			parent_id
		);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
			return;
		} else {
			toastSuccess('Group berhasil diupdate');
			back(true);
		}
	};

	onMount(async () => {
		const res = await getGroups(get(authStore).token ?? '');
		if (res.error) {
			toastError(res.error.error);
			return;
		}
		groups =
			res.result?.map((g: Group) => {
				return {
					value: g.id,
					name: g.name
				};
			}) || [];
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

<FormInput
	label="Nama group"
	bind:value={name}
	placeholder="Masukan nama group"
	valid={validName}
	helper="Nama group tidak boleh kosong"
	disabled={kind == 'view'}
/>
<FormInput
	multiline
	label="Deskripsi"
	bind:value={description}
	placeholder="Deskripsi group (opsional)"
	disabled={kind == 'view'}
/>
<div class="mb-6">
	<Label>
		Parent
		<Select
			class="mt-2"
			items={groups}
			bind:value={parent_id}
			placeholder="Pilih Parent (opsional)"
			disabled={kind == 'view'}
		/>
	</Label>
</div>
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
