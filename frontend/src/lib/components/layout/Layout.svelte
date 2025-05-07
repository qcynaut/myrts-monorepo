<script lang="ts">
	export let title: string = '';
	export let subtitle: string = '';
</script>

<div class="relative w-full p-2 md:p-4">
	<div class="mt-4">
		<h1 class="text-2xl font-bold">{title}</h1>
		<p>{subtitle}</p>
	</div>
	<slot />
</div>
