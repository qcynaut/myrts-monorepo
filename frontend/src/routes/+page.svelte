<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
	import IoTLogo from '$lib/components/images/IoTLogo.svelte';
	import CheckBox from '$lib/components/ui/CheckBox.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import { Icon } from '$lib/const/icon';
	import { auth, check } from '$lib/service/auth';
	import { authStore, setToken } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import { validateEmail, validatePassword } from '$lib/utils/validation';
	import { Button, Modal } from 'flowbite-svelte';
	import { CloseCircleSolid, ExclamationCircleOutline } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';

	let form = { email: '', password: '' };
	let showPassword: boolean = false;
	let keepMeLoggedIn: boolean = false;
	let emailValid: boolean = true;
	let emailError: string = 'Email tidak valid';
	let passwordValid: boolean = true;
	let passwordError: string = 'Password minimal 8 karakter';
	let popupModal = false;
	let popupError = '';
	let popupModalPending = false;

	authStore.subscribe((value) => {
		if (value.token && browser) {
			goto('/dashboard', { replaceState: true });
		}
	});

	onMount(async () => {
		if (browser) {
			let token = localStorage.getItem('token');
			if (token) {
				setLoading(true);
				const res = await check(token);
				setLoading(false);
				if (res.error) {
					localStorage.removeItem('token');
				} else {
					setToken(token);
				}
			}
		}
	});

	const handle = async () => {
		if (!validateEmail(form.email) || !validatePassword(form.password)) {
			emailValid = false;
			emailError = 'Email tidak valid';
			passwordValid = false;
			passwordError = 'Password minimal 8 karakter';
			return;
		}
		setLoading(true);
		const res = await auth(form.email, form.password);
		setLoading(false);
		if (res.error) {
			if (res.status == 404) {
				popupError = 'Email tidak ditemukan';
			} else {
				popupError = res.error.error;
			}
			popupModal = true;
		} else if (res.result?.pending) {
			popupModalPending = true;
		} else {
			localStorage.setItem('token', res.result?.token || '');
			setToken(res.result?.token);
		}
	};

	$: passwordType = showPassword ? 'text' : 'password';
</script>

<Modal bind:open={popupModal} size="xs">
	<div class="text-center">
		<CloseCircleSolid class="mx-auto mb-4 text-red-600 w-12 h-12" />
		<h3 class="mb-5 text-lg font-normal text-gray-500">{popupError}</h3>
		<Button
			color="red"
			class="mr-2"
			on:click={() => {
				popupModal = false;
			}}>Close</Button
		>
	</div>
</Modal>
<Modal bind:open={popupModalPending} size="xs">
	<div class="text-center">
		<ExclamationCircleOutline class="mx-auto mb-4 text-primary-500 w-12 h-12" />
		<h3 class="mb-5 text-lg font-normal text-gray-500">
			Kami mendeteksi adanya sesi aktif lain di akun anda, demi keamanan akun anda, silahkan
			konfirmasi aksi anda di email yang telah kami kirimkan.
		</h3>
		<Button
			color="primary"
			class="mr-2"
			on:click={() => {
				popupModalPending = false;
			}}>Close</Button
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
				<h1 class="text-3xl font-bold">Log in.</h1>
				<br />
				<p class="md:w-2/3">Login menggunakan akun yang telah di berikan oleh admin.</p>
				<br />
				<div class="mt-3">
					<label for="email" class="block text-lg font-medium text-gray-700">Email</label>
					<Input
						type="email"
						placeholder="Masukan email"
						id="email"
						bind:value={form.email}
						bind:valid={emailValid}
						validate={validateEmail}
						error={emailError}
					/>
				</div>
				<div class="mt-3">
					<label for="password" class="block text-lg font-medium text-gray-700">Password</label>
					<Input
						type={passwordType}
						placeholder="Masukan password"
						id="password"
						bind:value={form.password}
						bind:valid={passwordValid}
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
				<div class="mt-3 flex justify-between">
					<label for="remember" class="flex items-center">
						<CheckBox id="remember" bind:checked={keepMeLoggedIn} />
						<span class="ml-1 text-sm text-gray-600">Keep me logged in</span>
					</label>
					<a href="/forgot" class="text-sm text-primary-500">Forgot password?</a>
				</div>
				<div class="mt-12">
					<button
						on:click={handle}
						class="w-full px-3 py-2 text-white bg-primary-500 rounded-md shadow-sm focus:outline-none focus:ring-primary-500 focus:border-primary-500"
					>
						Login
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
