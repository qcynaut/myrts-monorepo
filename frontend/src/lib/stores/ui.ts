import { writable, type Writable } from 'svelte/store';

/**
 * Represents the UI Store.
 */
export type UIStore = {
	/** Indicates whether the UI is currently loading. */
	loading: boolean;
	/** Indicates whether the sidebar is expanded. */
	sidebarExpanded: boolean;
};

/**
 * The writable UI store.
 */
export const uiStore: Writable<UIStore> = writable({
	loading: false, // Indicates whether the UI is currently in a loading state.
	sidebarExpanded: true // Indicates whether the sidebar is expanded or collapsed.
});

/**
 * Sets the loading state of the UI.
 *
 * @param {boolean} loading - The loading state to set.
 */
export const setLoading = (loading: boolean) => {
	uiStore.update((state) => ({
		...state,
		loading
	}));
};

/**
 * Sets the sidebar expanded status.
 *
 * @param {boolean} expanded - The new expanded status of the sidebar.
 */
export const setSidebarExpanded = (expanded: boolean) => {
	uiStore.update((state) => ({
		...state,
		sidebarExpanded: expanded
	}));
};
