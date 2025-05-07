<script>
	import { goto } from '$app/navigation';
	import GroupForm from '$lib/components/form/GroupForm.svelte';
	import UserForm from '$lib/components/form/UserForm.svelte';
	import Layout from '$lib/components/layout/Layout.svelte';
	import { DASHBOARD } from '$lib/const/navigation';
	import { authStore } from '$lib/stores/auth';
	import { TabItem, Tabs } from 'flowbite-svelte';
	import { onMount } from 'svelte';

	onMount(() => {
		if ($authStore.user?.role?.id == 3) goto(DASHBOARD, { replaceState: true });
	});
</script>

<Layout title="User/Group" subtitle="Tambah User/Group">
	<div class="mt-4">
		<Tabs style="underline">
			<TabItem open title="User"><UserForm /></TabItem>
			<TabItem title="Group"><GroupForm /></TabItem>
		</Tabs>
	</div>
</Layout>
