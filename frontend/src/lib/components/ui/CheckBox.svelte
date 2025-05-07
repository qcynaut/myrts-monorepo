<script lang="ts">
	export let checked: boolean;
	export let id: string | undefined;
</script>

<label class="relative w-4 h-4" for={id}>
	<input {id} type="checkbox" class="hidden" bind:checked />
	<div class="absolute inset-0 ring-1 ring-gray-400 h-4 w-4 rounded" class:checked>
		{#if checked}
			<svg class="absolute inset-0 h-4 w-4" viewBox="0 0 20 20" fill="white">
				<path
					fill-rule="evenodd"
					d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
					clip-rule="evenodd"
				/>
			</svg>
		{/if}
	</div>
</label>

<style>
	.checked {
		@apply bg-primary-500 ring-transparent;
	}
</style>
