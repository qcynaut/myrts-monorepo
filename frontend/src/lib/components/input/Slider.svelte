<script lang="ts">
	export let min = 0;
	export let max = 1;
	export let value: number;
	export let step = 0.01;

	$: calcWidth = (value * 100 - 2) / max;
	$: width = isNaN(calcWidth) ? (max * 100) / max / 2 : calcWidth;
</script>

<div class="relative flex items-center flex-1">
	<div class="h-1 w-full rounded-sm bg-gray-300 absolute" />
	<input bind:value {min} {max} {step} type="range" class="slider" />
	<div class="h-1 rounded-sm bg-primary-500 absolute" style={`width: ${width}%;`} />
</div>

<style lang="scss">
	.slider {
		@apply w-full absolute h-2 appearance-none bg-transparent outline-none;

		&:-moz-range-thumb {
			@apply cursor-pointer relative z-10 bg-primary-500;
		}

		&::-webkit-slider-thumb {
			@apply cursor-pointer relative z-10 bg-primary-500;
		}
	}
</style>
