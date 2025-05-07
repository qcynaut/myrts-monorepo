<script lang="ts">
	import { goto } from '$app/navigation';
	import FormInput from '$lib/components/form/FormInput.svelte';
	import ModalSelect from '$lib/components/form/ModalSelect.svelte';
	import Layout from '$lib/components/layout/Layout.svelte';
	import AudioPlayer from '$lib/components/ui/AudioPlayer.svelte';
	import AudioRecorder from '$lib/components/ui/AudioRecorder.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import MiniPlayer from '$lib/components/ui/MiniPlayer.svelte';
	import { Icon } from '$lib/const/icon';
	import { DASHBOARD_RECORD_LIST } from '$lib/const/navigation';
	import { get_group_paginated } from '$lib/service/group';
	import { createRecord } from '$lib/service/record';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import type { GroupData } from '$lib/types/response';
	import { toastError } from '$lib/utils/toast';
	import { Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';

	let name = '';
	let user_ids: number[] = [];
	let description = '';
	let file: File | undefined;
	let fileElement: HTMLInputElement;
	let users: { value: number; name: string }[] = [];
	let validName = true;
	let recording = false;

	const getNestedUser = (groups: GroupData[]) => {
		let users: { value: number; name: string }[] = [];
		for (const group of groups) {
			users = [...users, ...group.users.map((u) => ({ value: u.id, name: u.name }))];
			if (group.children.length > 0) {
				users = [...users, ...getNestedUser(group.children)];
			}
		}
		return users;
	};

	const onUpload = (e: Event & { currentTarget: EventTarget & HTMLInputElement }) => {
		if (e.currentTarget.files) {
			let size = e.currentTarget.files[0].size;
			// max 1GB
			if (size > 1024 * 1024 * 1024) {
				toastError('File terlalu besar');
				return;
			}
			file = e.currentTarget.files[0];
		}
	};

	const submit = async () => {
		if (name == '') return (validName = false);
		if (!file) return;
		setLoading(true);
		const res = await createRecord(
			get(authStore).token || '',
			name,
			description == '' ? null : description,
			user_ids,
			file
		);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		} else {
			goto(DASHBOARD_RECORD_LIST, { replaceState: true });
		}
	};

	const loadUser = async () => {
		setLoading(true);
		let total_page = 1;
		let current_page = 1;
		const res = await get_group_paginated(1, get(authStore).token || '');
		if (res.result?.items) {
			total_page = res.result.total_page;
			for (const item of res.result.items) {
				users = [
					...users,
					...item.users.map((u) => {
						return { value: u.id, name: u.name };
					})
				];
				users = [...users, ...getNestedUser(item.children)];
			}
			while (current_page < total_page) {
				current_page++;
				const res = await get_group_paginated(current_page, get(authStore).token || '');
				if (res.result?.items) {
					total_page = res.result.total_page;
					for (const item of res.result.items) {
						users = [
							...users,
							...item.users.map((u) => {
								return { value: u.id, name: u.name };
							})
						];
						users = [...users, ...getNestedUser(item.children)];
					}
				}
			}
		}
		setLoading(false);
		if ($authStore.user) {
			users = users.filter((u) => u.value != $authStore.user?.id);
		}
	};

	const startRecording = async () => {
		recording = true;
	};

	onMount(() => {
		loadUser();
	});
</script>

{#if recording}
	<div
		class="fixed w-screen h-screen left-0 top-0 z-20 bg-black bg-opacity-50 justify-center items-center flex"
	>
		<AudioRecorder cancel={() => (recording = false)} save={(f) => (file = f)} />
	</div>
{/if}

<Layout title="Rekaman" subtitle="Tambah/Kirim Rekaman">
	<div class="mt-4">
		<div class="mb-8 flex justify-end">
			<button
				on:click={startRecording}
				class="px-4 py-2 flex gap-2 items-center bg-primary-500 rounded-l-full rounded-r-full text-white"
			>
				<Icons name={Icon.Microphone} />
				Rekam
			</button>
		</div>
		<FormInput label="Judul" placeholder="Judul rekaman" bind:value={name} valid={validName} />
		<ModalSelect label="User tujuan (opsional)" bind:value={user_ids} items={users} />
		<FormInput
			multiline
			label="Deskripsi"
			placeholder="Deskripsi rekaman"
			bind:value={description}
		/>
		<label for="file" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">
			Rekaman
		</label>
		<input
			type="file"
			id="file"
			accept="audio/mpeg"
			on:change={onUpload}
			class="hidden"
			bind:this={fileElement}
		/>
		<div
			class="cursor-pointer mt-2 w-full flex justify-center items-center shadow-xl p-8 rounded-lg"
		>
			<div class="flex justify-center items-center">
				<div class="relative">
					{#if file}
						<button
							on:click={() => (file = undefined)}
							class="absolute -top-2 -right-2 text-white bg-red-500 rounded-full text-xl w-8 h-8"
						>
							<Icons name={Icon.Times} />
						</button>
					{/if}
					<button
						on:click={() => fileElement.click()}
						class="p-4 rounded-lg border border-dashed border-gray-500 bg-gray-200"
					>
						<Icons name={Icon.Upload} class="text-4xl text-primary-400" />
						{#if !file}
							<p class="mt-2 text-gray-500">Pilih file rekaman</p>
						{:else}
							<p class="mt-2 text-white p-2 rounded-md bg-primary-400">{file.name}</p>
						{/if}
					</button>
				</div>
			</div>
		</div>
		{#if file}
			<div class="flex justify-center">
				<AudioPlayer src={URL.createObjectURL(file)}>
					<MiniPlayer />
				</AudioPlayer>
			</div>
		{/if}
		<div class="my-6 flex justify-center">
			<Button on:click={submit}>Submit</Button>
		</div>
	</div>
</Layout>
