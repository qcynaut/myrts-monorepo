<script lang="ts">
	import { twMerge } from 'tailwind-merge';

	export let shadow: 'sm' | 'md' | 'lg' | 'xl' = 'md';
	export let rounded: 'sm' | 'md' | 'lg' | 'xl' = 'md';
	export { className as class };

	let className = '';
	let classes = `w-full h-full overflow-auto shadow-${shadow} rounded-${rounded} ${className}`;
</script>

<div class={twMerge(classes)}>
	<slot />
</div>
