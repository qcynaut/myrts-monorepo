<script lang="ts">
	import Layout from '$lib/components/layout/Layout.svelte';
	import { Img, Modal } from 'flowbite-svelte';
	import Avatar from '../../../assets/avatar.png';
	import Icons from '$lib/components/ui/Icons.svelte';
	import { Icon } from '$lib/const/icon';
	import { toastError, toastSuccess } from '$lib/utils/toast';
	import { patchImage } from '$lib/service/user';
	import { authStore } from '$lib/stores/auth';
	import { setLoading } from '$lib/stores/ui';
	import { utcToLocal } from '$lib/utils/format';

	let open = false;
	let input: HTMLInputElement;
	let videoElement: HTMLVideoElement;
	let mediaStream: MediaStream | null = null;
	let file: File | null = null;

	const openCamera = async () => {
		try {
			mediaStream = await navigator.mediaDevices.getUserMedia({ video: true });
			videoElement.srcObject = mediaStream;
			videoElement.play();
		} catch (error) {
			toastError((error as Error).message);
		}
	};

	const onChange = (event: Event) => {
		file = (event.target as HTMLInputElement).files?.[0] || null;
	};

	const takePhoto = async () => {
		const canvas = document.createElement('canvas');
		const context = canvas.getContext('2d');
		canvas.width = videoElement.videoWidth;
		canvas.height = videoElement.videoHeight;
		// Flip the image horizontally
		context!.scale(-1, 1);
		context!.drawImage(videoElement, -canvas.width, 0, canvas.width, canvas.height);
		canvas.toBlob((blob) => {
			file = new File([blob!], 'avatar.png', {
				type: 'image/png'
			});
		}, 'image/png');

		mediaStream?.getTracks().forEach((track) => track.stop());
		mediaStream = null;
	};

	const cancel = () => {
		mediaStream?.getTracks().forEach((track) => track.stop());
		mediaStream = null;
	};

	const submit = async () => {
		if (!file) return;
		open = false;
		setLoading(true);
		const res = await patchImage($authStore.token || '', file);
		setLoading(false);
		if (res.error) {
			toastError(res.error.error);
		} else {
			toastSuccess('Berhasil memperbarui profil');
			authStore.update((auth) => {
				if (auth.user && res.result?.image_url) {
					auth.user.image_url = res.result.image_url;
				}
				return auth;
			});
		}
	};

	$: user = $authStore.user;
	$: avatar = user?.image_url || Avatar;
</script>

<div
	class="fixed inset-0 bg-black bg-opacity-80 w-screen h-screen z-[9999] p-8 {mediaStream
		? 'flex'
		: 'hidden'} items-center justify-center"
>
	<div class="relative">
		<!-- svelte-ignore a11y-media-has-caption -->
		<video bind:this={videoElement} autoplay playsinline class="transform scale-x-[-1]" />
		<button
			on:click={takePhoto}
			class="absolute bottom-10 left-1/2 transform translate-x-[-50%] text-gray-400 bg-gray-200 rounded-full text-2xl w-10 h-10"
			><Icons name={Icon.Camera} /></button
		>
		<button
			on:click={cancel}
			class="absolute top-5 right-5 text-gray-400 bg-gray-200 rounded-full text-2xl w-10 h-10"
		>
			<Icons name={Icon.Times} />
		</button>
	</div>
</div>

<Layout title="Profile">
	<div class="mt-4 p-6 rounded-lg bg-white shadow-lg">
		<div class="flex justify-center">
			<div class="relative">
				<Img src={avatar} size="w-32" class="rounded-full h-32" />
				<button
					on:click={() => (open = true)}
					class="absolute bottom-0 right-0 text-gray-400 bg-gray-200 rounded-full text-2xl w-10 h-10"
					><Icons name={Icon.Camera} /></button
				>
				<Modal bind:open size="xs">
					<h1 class="font-bold">Ganti Foto</h1>
					<div class="flex flex-col gap-2">
						<input
							type="file"
							accept="image/*"
							class="hidden"
							bind:this={input}
							on:change={onChange}
						/>
						{#if file}
							<Img src={URL.createObjectURL(file)} />
							<div class="flex gap-2 justify-end">
								<button class="px-4 py-2 rounded-md bg-primary-500 text-white" on:click={submit}
									>Simpan</button
								>
								<button
									on:click={() => (file = null)}
									class="px-4 py-2 rounded-md bg-red-500 text-white">Hapus</button
								>
							</div>
						{:else}
							<button
								class="text-xl text-gray-500 flex gap-2 items-center"
								on:click={() => input.click()}
								><Icons name={Icon.Image} /> <span>Pilih dari perangkat</span></button
							>
							<hr />
							<button class="text-xl text-gray-500 flex gap-2 items-center" on:click={openCamera}
								><Icons name={Icon.Camera} /> <span>Ambil dari kamera</span></button
							>
						{/if}
					</div>
				</Modal>
			</div>
		</div>
		<div class="mb-6">
			<h1 class="text-lg mb-4">Nama</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				<Icons name={Icon.User} /><span>{user?.name}</span>
			</div>
		</div>
		<div class="mb-6">
			<h1 class="text-lg mb-4">Paket user terpilih</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				<Icons name={Icon.Box} /><span>{user?.subscription?.package.name || '-'}</span>
			</div>
		</div>
		<div class="mb-6">
			<h1 class="text-lg mb-4">Tanggal terdaftar</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				<Icons name={Icon.Calendar} /><span
					>{user?.subscription ? utcToLocal(user.subscription.order_date) : '-'}</span
				>
			</div>
		</div>
		<div class="mb-6">
			<h1 class="text-lg mb-4">Tanggal berakhir</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				<Icons name={Icon.Calendar} /><span
					>{user?.subscription ? utcToLocal(user.subscription.expire_date) : '-'}</span
				>
			</div>
		</div>
		<div class="mb-6">
			<h1 class="text-lg mb-4">Email</h1>
			<div class="p-2 rounded-lg border border-gray-400 flex gap-2 items-center">
				<Icons name={Icon.Envelope} /><span>{user?.email}</span>
			</div>
		</div>
	</div>
</Layout>
