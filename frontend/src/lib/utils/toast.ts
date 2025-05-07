import { toast } from '@zerodevx/svelte-toast';

/**
 * Displays a success toast message.
 *
 * @param {string} message - The message to be displayed in the toast.
 */
export const toastSuccess = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastColor': 'white',
			'--toastBackground': '#16a34a',
			'--toastBarBackground': '#052e16'
		}
	});

/**
 * Displays a warning toast message with the given message.
 *
 * @param {string} message - The message to be displayed in the warning toast.
 */
export const toastWarnig = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastColor': 'white',
			'--toastBackground': '#ca8a04',
			'--toastBarBackground': '#a16207'
		}
	});

/**
 * Displays an error toast message with the given message.
 *
 * @param {string} message - The error message to display.
 */
export const toastError = (message: string) =>
	toast.push(message, {
		theme: {
			'--toastColor': 'white',
			'--toastBackground': '#dc2626',
			'--toastBarBackground': '#991b1b'
		}
	});
