<script lang="ts">
	import logo from '../../../assets/logo.png';
	import logoSmall from '../../../assets/logo-small.png';

	let className: string = '';
	export { className as class };
	export let height: string | number | undefined = undefined;
	export let width: string | number | undefined = undefined;
	export let variant: 'small' | 'large' = 'large';
</script>

{#if variant == 'large'}
	<img src={logo} alt="MyRTS IoT Logo" class={className} {height} {width} />
{:else}
	<img src={logoSmall} alt="MyRTS IoT Logo" class={className} {height} {width} />
{/if}
