import { writable, type Writable } from 'svelte/store';

/**
 * Represents a single toast item.
 */
export type ToastItem = {
	type: 'success' | 'info' | 'warning' | 'error'; // The type of the toast item
	message: string; // The message to display in the toast
	id: string; // A unique identifier for the toast
};

/**
 * Represents a store of toast items.
 */
export type ToastStore = {
	toasts: ToastItem[]; // An array of toast items
};

// Define a writable store for managing toasts
export const toastStore: Writable<ToastStore> = writable({
	// Initialize the toasts array to an empty array
	toasts: []
});

/**
 * Notifies the user with a toast message.
 *
 * @param {string} message - The message to be displayed in the toast.
 * @param {'success' | 'info' | 'warning' | 'error'} type - The type of the toast: 'success', 'info',
 * 'warning', or 'error'.
 */
export function notify(message: string, type: 'success' | 'info' | 'warning' | 'error') {
	toastStore.update((store) => {
		const id =
			Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
		store.toasts = [...store.toasts, { type, message, id }];
		return store;
	});
}

/**
 * Removes a toast from the toast store.
 *
 * @param {ToastItem} toast - The toast item to be removed.
 */
export function removeToast(toast: ToastItem) {
	toastStore.update((store) => {
		store.toasts = store.toasts.filter((t) => t !== toast);
		return store;
	});
}
