<script lang="ts">
	import Layout from '$lib/components/layout/Layout.svelte';
	import AudioPlayer from '$lib/components/ui/AudioPlayer.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import LargePlayer from '$lib/components/ui/LargePlayer.svelte';
	import { Icon } from '$lib/const/icon';
	import { acceptRecord, deleteRecords, getRecords } from '$lib/service/record';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import type { Records } from '$lib/types/response';
	import { formatDurationToString, utcToLocal } from '$lib/utils/format';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import {
		Button,
		Modal,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { get } from 'svelte/store';
	import { utils, writeFileXLSX } from 'xlsx';

	let search = '';
	let el: HTMLDivElement;
	let records: Records[] = [];
	let playId: number | null = null;
	let paused = true;
	let seek = false;
	let deleteId: number | null = null;

	const exportRecord = () => {
		const doc = el.getElementsByTagName('table')[0];
		const wb = utils.table_to_book(doc);
		writeFileXLSX(wb, 'records.xlsx');
	};

	const load = async () => {
		setLoading(true);
		const res = await getRecords(get(authStore).token || '');
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		}
		records = res.result?.filter((r) => r.status == 0) || [];
	};

	const accept = async (id: number) => {
		setLoading(true);
		const res = await acceptRecord(get(authStore).token || '', id);
		setLoading(false);
		if (res.error) {
			toastError('Gagal menerima rekaman');
		} else {
			records = records.filter((r) => r.id != id);
			toastSuccess('Rekaman berhasil diterima');
		}
	};

	const play = (id: number) => {
		if (playId == null) return (playId = id);
		if (id != playId) {
			seek = true;
			playId = id;
		}
		paused = false;
	};
	const pause = () => {
		paused = true;
	};

	const next = () => {
		if (playId == null || playId + 1 >= records.length) {
			return;
		}
		play(playId + 1);
	};

	const prev = () => {
		if (playId && playId - 1 >= 0) {
			play(playId - 1);
		} else {
			seek = true;
		}
	};

	const getIndex = (record: Records) => {
		return records.findIndex((r) => r.id == record.id);
	};

	const removeRecord = async () => {
		if (deleteId != null && records[deleteId]) {
			const record = records[deleteId];
			deleteId = null;
			setLoading(true);
			const res = await deleteRecords(get(authStore).token || '', record.id);
			setLoading(false);
			if (res.error) {
				toastError('Gagal menolak rekaman');
			} else {
				records = records.filter((r) => r.id != record.id);
				toastSuccess('Rekaman berhasil ditolak');
			}
		}
	};

	onMount(() => {
		load();
	});

	$: filtered =
		search == ''
			? records
			: records.filter((r) => r.name.toLowerCase().includes(search.toLowerCase()));
	$: user = $authStore.user;
</script>

{#if deleteId != null}
	<Modal title="Tolak rekaman" on:close={() => (deleteId = null)} open>
		<p>Apakah anda yakin ingin menolak rekaman ini?</p>
		<div class="mt-4 flex justify-end gap-4">
			<Button on:click={() => (deleteId = null)}>Cancel</Button>
			<Button color="red" on:click={removeRecord}>Tolak</Button>
		</div>
	</Modal>
{/if}

<div class="absolute w-full bottom-0 z-10 {playId != null ? '' : 'hidden'}">
	<AudioPlayer src={playId != null ? records[playId].file_url : ''}>
		<LargePlayer bind:seek bind:playerPaused={paused} {prev} {next} close={() => (playId = null)} />
	</AudioPlayer>
</div>

<Layout title="Rekaman" subtitle="Rekaman Pending">
	<div class="mt-4 flex justify-between gap-8 items-center">
		<button
			class="flex items-center gap-2 text-white py-2 px-4 bg-primary-500 rounded-l-full rounded-r-full"
			on:click={exportRecord}><span>export</span><Icons name={Icon.AngleDown} /></button
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
	<div class="mt-4 mb-8" bind:this={el}>
		<Table shadow={true}>
			<TableHead class="text-xs">
				<TableHeadCell />
				<TableHeadCell>Nama</TableHeadCell>
				<TableHeadCell>Durasi</TableHeadCell>
				<TableHeadCell>Uploaded At</TableHeadCell>
				<TableHeadCell>Pengirim</TableHeadCell>
				<TableHeadCell>Deskripsi</TableHeadCell>
				<TableHeadCell>Action</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each filtered as record, i}
					<TableBodyRow class={playId == getIndex(record) ? 'bg-primary-300' : ''}>
						<TableBodyCell>
							<span class="rec-num" class:played={playId == getIndex(record)}>
								<span class="num">{i + 1}</span>
								<span class="action">
									<button
										on:click={() => {
											const index = getIndex(record);
											if (playId == index) {
												if (paused) {
													play(index);
												} else {
													pause();
												}
											} else {
												play(index);
											}
										}}
										><Icons
											name={playId == getIndex(record)
												? paused
													? Icon.Play
													: Icon.Pause
												: Icon.Play}
										/></button
									>
								</span>
							</span>
						</TableBodyCell>
						<TableBodyCell>{record.name}</TableBodyCell>
						<TableBodyCell>{formatDurationToString(parseInt(record.duration))}</TableBodyCell>
						<TableBodyCell>{utcToLocal(record.created_at)}</TableBodyCell>
						<TableBodyCell>{record.sender?.name ?? ''}</TableBodyCell>
						<TableBodyCell>{record.description ?? ''}</TableBodyCell>
						<TableBodyCell>
							<div class="flex">
								<button
									on:click={() => accept(record.id)}
									class="w-6 h-6 flex justify-center items-center rounded-full bg-primary-500"
								>
									<Icons name={Icon.Check} class="text-white" />
								</button>
								{#if user?.id == record.user_id || user?.role_id == 1 || user?.role_id == 2}
									<button
										on:click={() => (deleteId = getIndex(record))}
										class="w-6 h-6 flex justify-center items-center rounded-full bg-red-500"
									>
										<Icons name={Icon.Times} class="text-white" />
									</button>
								{/if}
							</div>
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</Layout>

<style lang="scss">
	.rec-num {
		@apply cursor-pointer p-2;

		.action {
			@apply hidden;
		}

		&:hover {
			.num {
				@apply hidden;
			}
			.action {
				@apply inline;
			}
		}
		&.played {
			.num {
				@apply hidden;
			}
			.action {
				@apply inline;
			}
		}
	}
</style>
