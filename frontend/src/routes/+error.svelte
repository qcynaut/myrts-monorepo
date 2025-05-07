<script lang="ts">
	import { page } from '$app/stores';
	import IoTLogo from '$lib/components/images/IoTLogo.svelte';
	import Icons from '$lib/components/ui/Icons.svelte';
	import { Icon } from '$lib/const/icon';
</script>

<div class="w-screen h-screen bg-gray-200 p-2 md:p-4 flex justify-center items-center">
	<div class="p-4 w-full md:w-1/3 bg-white rounded-lg shadow-lg">
		<div class="my-2 px-4">
			<IoTLogo />
		</div>
		<div class="my-4 flex justify-center items-center">
			<Icons name={Icon.ExclamationCircle} class="text-6xl text-red-600" />
		</div>
		<h1 class="mt-4 text-center text-4xl font-bold">Error</h1>
		<div class="mt-2 p-2 rounded bg-gray-100 text-center italic">
			{$page.error?.message}
		</div>
	</div>
</div>
