<script lang="ts">
	import IoTLogo from '$lib/components/images/IoTLogo.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import { Icon } from '$lib/const/icon';
	import { validatePassword } from '$lib/utils/validation';
	import { Button, Modal } from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { goto } from '$app/navigation';
	import { CheckCircleOutline, CloseCircleSolid } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { resetPassword } from '$lib/service/auth';
	import { setLoading } from '$lib/stores/ui';

	export let data: PageData;

	let password = '';
	let validPassword = true;
	let passwordError = 'Password minimal 8 karakter';
	let showPassword: boolean = false;

	let popupModal = false;
	let popupText = 'Link reset password tidak di temukan / kadaluwarsa.';
	let popupError = true;

	const handle = async () => {
		if (!validatePassword(password)) {
			validPassword = false;
			passwordError = 'Password minimal 8 karakter';
			return;
		}
		setLoading(true);
		const res = await resetPassword(data.uuid, password);
		setLoading(false);
		if (res.error) {
			popupText = res.error.error;
			popupModal = true;
		} else {
			popupText = 'Password berhasil di reset';
			popupError = false;
			popupModal = true;
		}
	};

	onMount(() => {
		if (data.error) {
			popupModal = true;
		}
	});

	$: passwordType = showPassword ? 'text' : 'password';
</script>

<Modal
	bind:open={popupModal}
	size="xs"
	on:close={() => {
		goto('/', { replaceState: true });
	}}
>
	<div class="text-center">
		{#if popupError}
			<CloseCircleSolid class="mx-auto mb-4 text-red-600 w-12 h-12" />
		{:else}
			<CheckCircleOutline class="mx-auto mb-4 text-primary-500 w-12 h-12" />
		{/if}
		<h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
			{popupText}
		</h3>
		<Button
			color={popupError ? 'red' : 'primary'}
			class="mr-2"
			on:click={() => {
				goto('/', { replaceState: true });
			}}>Kembali</Button
		>
	</div>
</Modal>

<div class="w-screen h-screen bg-white md:flex overflow-hidden">
	<section class="p-5 md:w-2/5 md:mt-12">
		<div class="relative rounded-xl md:p-4 md:shadow-2xl">
			<div class="relative w-full p-8">
				<IoTLogo
					class="md:absolute md:w-2/3 md:inset-0 md:-top-12 md:left-1/2 md:transform md:-translate-x-1/2"
				/>
			</div>
			<div class="mt-5">
				<h1 class="text-3xl font-bold">Reset password</h1>
				<br />
				<p class="md:w-2/3">Silahkan masukan password baru.</p>
				<br />
				<div class="mt-3">
					<label for="password" class="block text-lg font-medium text-gray-700">Password</label>
					<Input
						type={passwordType}
						placeholder="Masukan password"
						id="password"
						bind:value={password}
						bind:valid={validPassword}
						validate={validatePassword}
						error={passwordError}
						endItem
					>
						<svelte:fragment slot="endItem">
							<Icons
								class="cursor-pointer"
								name={!showPassword ? Icon.Eye : Icon.EyeSlash}
								on:click={() => (showPassword = !showPassword)}
							/>
						</svelte:fragment>
					</Input>
				</div>
				<div class="mt-6">
					<button
						on:click={handle}
						class="w-full px-3 py-2 text-white bg-primary-500 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
					>
						Atur ulang
					</button>
					<button
						on:click={() => {
							goto('/', { replaceState: true });
						}}
						class="mt-6 w-full px-3 py-2 flex gap-4 items-center justify-center text-gray-500 rounded-md shadow-sm focus:outline-none"
					>
						<Icons name={Icon.ArrowLeft} />
						<span>Back to login</span>
					</button>
				</div>
			</div>
		</div>
		<div class="mt-8 w-full p-5 text-md text-center text-gray-500">
			<h1 class="text-center">Powered by TIGA PILAR GLOBAL</h1>
		</div>
	</section>
	<section class="relative hidden md:block md:w-3/5 md:h-full bg-primary-500">
		<div class="absolute flex gap-4 bottom-5 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
			<div class="w-4 h-4 bg-white rounded-full" />
			<div class="w-4 h-4 bg-gray-400 rounded-full" />
			<div class="w-4 h-4 bg-gray-400 rounded-full" />
			<div class="w-4 h-4 bg-gray-400 rounded-full" />
		</div>
	</section>
</div>
