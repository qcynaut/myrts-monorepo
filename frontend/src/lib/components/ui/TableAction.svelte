<script lang="ts">
	import { authStore } from '$lib/stores/auth';
	import { Button, Modal, TableBodyCell } from 'flowbite-svelte';
	import Icons from './Icons.svelte';
	import { Icon } from '$lib/const/icon';

	type VoidFn = () => void;

	export let view: VoidFn = () => {};
	export let edit: VoidFn = () => {};
	export let remove: VoidFn | undefined = undefined;
	export let removeConfirm: string = '';

	let open = false;

	$: user = $authStore.user;
</script>

<div class="absolute">
	<Modal bind:open>
		<h1 class="text-center text-2xl">Peringatan</h1>
		<p class="text-center">
			{removeConfirm}
			<br />
			Ini akan menghapus semua data yang terkait dengan user ini.
		</p>
		<div class="flex justify-end gap-2">
			<Button
				color="red"
				on:click={() => {
					open = false;
					if (remove) {
						remove();
					}
				}}>Ya</Button
			>
			<Button color="alternative" on:click={() => (open = false)}>Tidak</Button>
		</div>
	</Modal>
</div>

<TableBodyCell tdClass="px-0" class="flex">
	<button
		on:click={view}
		class="w-6 h-6 flex justify-center items-center rounded-full bg-green-600"
	>
		<Icons name={Icon.Eye} class="text-white" />
	</button>
	{#if user?.role?.id != 3}
		<button
			on:click={edit}
			class="w-6 h-6 flex justify-center items-center rounded-full bg-primary-500"
		>
			<Icons name={Icon.Pencil} class="text-white" />
		</button>
		{#if remove}
			<button
				class="w-6 h-6 flex justify-center items-center rounded-full bg-red-600"
				on:click={() => (open = true)}
			>
				<Icons name={Icon.Trash} class="text-white" />
			</button>
		{/if}
	{/if}
</TableBodyCell>
