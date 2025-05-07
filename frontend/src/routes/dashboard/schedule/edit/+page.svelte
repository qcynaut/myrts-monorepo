<script lang="ts">
	import { goto } from '$app/navigation';
	import { DASHBOARD_SCHEDULE } from '$lib/const/navigation';
	import { onMount } from 'svelte';

	onMount(() => {
		goto(DASHBOARD_SCHEDULE, { replaceState: true });
	});
</script>
