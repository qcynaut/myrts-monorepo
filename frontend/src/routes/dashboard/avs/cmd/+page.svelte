<script lang="ts">
	import { browser } from '$app/environment';
	import Button from '$lib/components/button/Button.svelte';
	import Input from '$lib/components/input/Input.svelte';
	import Label from '$lib/components/input/Label.svelte';
	import DashboardInner from '$lib/components/layout/DashboardInner.svelte';
	import { STREAM_URL } from '$lib/const/urls';
	import Command from '$lib/rtc/command';
	import { authStore } from '$lib/stores/auth';
	import { onMount } from 'svelte';

	let cmd = '';
	let target = '';
	let result = '';

	let instance: Command | null = null;

	onMount(async () => {
		if (browser) {
			instance = await Command.create(STREAM_URL);
			instance.start($authStore.token ?? '');
			instance.response_handler = (res) => {
				result = res + '\n';
			};
		}
	});

	const send = () => {
		const user = $authStore.user?.id;
		instance?.command(cmd, target, user ?? 0);
		cmd = '';
	};
</script>

<DashboardInner title="AVS Command">
	<div class="w-full flex flex-col flex-1 bg-white p-4 overflow-hidden">
		<div class="flex flex-col gap-2 items-center">
			<Label>
				Target
				<Input type="text" bind:value={target} />
			</Label>
			<Label>
				Command
				<Input type="text" bind:value={cmd} />
			</Label>
			<Button class="text-white bg-primary-600" on:click={send}>Send</Button>
		</div>
		<div class="flex flex-1 h-full justify-center">
			<div class="w-1/3 h-full overflow-y-auto">
				<div class="mt-4 whitespace-pre-line">
					{result}
				</div>
			</div>
		</div>
	</div>
</DashboardInner>
