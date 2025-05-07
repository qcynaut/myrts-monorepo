/** @type {import('tailwindcss').Config} */
export default {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],
	plugins: [require('flowbite/plugin')],
	theme: {
		extend: {
			colors: {
				primary: {
					50: '#f2f3fc',
					100: '#e3e5f6',
					200: '#cdd3f0',
					300: '#aab4e6',
					400: '#818ed9',
					500: '#636cce',
					600: '#4c4ebf',
					700: '#4945b0',
					800: '#423d90',
					900: '#373573',
					950: '#262447'
				}
			}
		}
	}
};
