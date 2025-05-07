<script lang="ts">
	import type { Icon } from '$lib/const/icon';
	import Icons from './Icons.svelte';

	let className: string = '';
	export { className as class };
	export let icon: Icon;
	export let color: 'darkBlue' | 'blue' | 'green' | 'red' | 'yellow' | 'gray' = 'gray';
	export let title: string = '';
	export let value = '0';

	const colorMap = {
		background: {
			darkBlue: 'after-darkBlue',
			blue: 'after-blue',
			green: 'after-green',
			red: 'after-red',
			yellow: 'after-yellow',
			gray: 'after-gray'
		},
		text: {
			darkBlue: 'darkBlue',
			blue: 'blue',
			green: 'green',
			red: 'red',
			yellow: 'yellow',
			gray: 'gray'
		}
	};
</script>

<div class="dashboard-box w-full border border-gray-200 rounded-lg py-4 {className}">
	<div class="relative h-full w-full p-2 {colorMap.background[color]}" id="box-inner">
		<div class="h-full w-full px-2 md:px-4 flex gap-4 items-center justify-center">
			<div
				class="w-8 md:w-12 h-8 md:h-12 p-2 shadow-md rounded-full flex justify-center items-center"
			>
				<Icons name={icon} class="text-2xl md:text-3xl {colorMap.text[color]}" />
			</div>
			<div class="flex flex-col">
				<h1 class="text-gray-500 text-sm font-medium">{title}</h1>
				<p>{value}</p>
			</div>
		</div>
	</div>
</div>

<style lang="scss">
	#box-inner::after {
		content: '';
		display: block;
		@apply absolute left-0 top-1/2 transform -translate-y-1/2 w-1 h-full rounded-r-lg;
	}

	.dashboard-box {
		:global(.darkBlue) {
			@apply text-blue-800;
		}

		:global(.blue) {
			@apply text-blue-500;
		}

		:global(.green) {
			@apply text-green-500;
		}

		:global(.red) {
			@apply text-red-600;
		}

		:global(.yellow) {
			@apply text-yellow-500;
		}

		:global(.gray) {
			@apply text-gray-500;
		}
	}

	.after-darkBlue::after {
		@apply bg-blue-800;
	}

	.after-blue::after {
		@apply bg-blue-500;
	}

	.after-green::after {
		@apply bg-green-500;
	}

	.after-red::after {
		@apply bg-red-600;
	}

	.after-yellow::after {
		@apply bg-yellow-500;
	}

	.after-gray::after {
		@apply bg-gray-500;
	}
</style>
