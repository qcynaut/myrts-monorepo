<script lang="ts">
	export let min = 0;
	export let max = 1;
	export let value: number;
	export let step = 0.01;
</script>

<div class="relative flex items-center flex-1">
	<div class="h-1 rounded-sm bg-primary-800 absolute" style={`width: ${(value * 100) / max}%;`} />
	<div class="h-1 w-full rounded-sm bg-primary-300 absolute" />
	<input
		bind:value
		{min}
		{max}
		{step}
		type="range"
		class="w-full absolute h-2 appearance-none bg-transparent outline-none"
	/>
</div>

<style lang="scss">
	input[type='range'] {
		-webkit-appearance: none;
	}
	input[type='range']::-webkit-slider-runnable-track {
		@apply bg-transparent;
		height: 5px;
		border: none;
		border-radius: 3px;
	}
	input[type='range']::-webkit-slider-thumb {
		@apply bg-primary-500;
		-webkit-appearance: none;
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 50%;
		margin-top: -4px;
	}
	input[type='range']:focus {
		outline: none;
	}
	input[type='range']:focus::-webkit-slider-runnable-track {
		@apply bg-transparent;
	}
	input[type='range']::-moz-range-track {
		@apply bg-transparent;
		height: 5px;
		border: none;
		border-radius: 3px;
	}
	input[type='range']::-moz-range-thumb {
		@apply bg-primary-500;
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 50%;
	}

	/*hide the outline behind the border*/
	input[type='range']:-moz-focusring {
		outline-offset: -1px;
	}
	input[type='range']:focus::-moz-range-track {
		@apply bg-transparent;
	}

	/* for ie */

	input[type='range']::-ms-track {
		height: 5px;
		background: transparent;
		border-color: transparent;
		border-width: 6px 0;
		color: transparent;
	}
	input[type='range']::-ms-fill-lower {
		@apply bg-transparent;
		border-radius: 10px;
	}
	input[type='range']::-ms-fill-upper {
		@apply bg-transparent;
		border-radius: 10px;
	}
	input[type='range']::-ms-thumb {
		@apply bg-primary-500;
		border: none;
		height: 16px;
		width: 16px;
		border-radius: 50%;
	}
	input[type='range']:focus::-ms-fill-lower {
		@apply bg-transparent;
	}
	input[type='range']:focus::-ms-fill-upper {
		@apply bg-transparent;
	}
</style>
