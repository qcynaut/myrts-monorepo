<script lang="ts">
	export { className as class };
	export { htmlFor as for };

	let className = '';
	let htmlFor = '';
</script>

<label for={htmlFor == '' ? undefined : htmlFor} class="{className} block text-md mb-4">
	<slot />
</label>
