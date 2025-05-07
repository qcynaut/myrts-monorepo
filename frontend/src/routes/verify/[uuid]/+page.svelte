<script lang="ts">
	import { goto } from '$app/navigation';
	import IoTLogo from '$lib/components/images/IoTLogo.svelte';
	import { Button } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	export let data: PageData;

	onMount(() => {
		if (!browser) {
			return;
		}
		if (data.result) {
			if (!data.result.mobile) localStorage.setItem('token', data.result.token);
		}
	});
</script>

<div class="w-screen h-screen bg-white flex justify-center items-center p-2">
	<div class="w-full md:w-1/3 rounded-xl shadow-md p-4">
		<div class="relative w-full p-8">
			<IoTLogo
				class="md:absolute md:w-2/3 md:inset-0 md:-top-12 md:left-1/2 md:transform md:-translate-x-1/2"
			/>
		</div>
		<p class="text-lg">
			{#if data.error}
				Sesi tidak di temukan atau link kadaluwarsa.
			{:else}
				Sesi anda telah di verifikasi, silahkan kembali ke aplikasi.
			{/if}
		</p>
		<div class="mt-5 w-full text-center">
			<Button
				color={data.error ? 'red' : 'primary'}
				class="mr-2"
				on:click={() => {
					goto('/', { replaceState: true });
				}}>Kembali</Button
			>
		</div>
	</div>
</div>
