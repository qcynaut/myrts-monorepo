import type { User } from '$lib/types/response';
import { get, writable, type Writable } from 'svelte/store';

/**
 * Represents the authentication store.
 */
export type AuthStore = {
	/**
	 * The authentication token.
	 */
	token: string | null;

	/**
	 * The user data.
	 */
	user: User | null;
};

/**
 * The writable store for the authentication store.
 */
export const authStore: Writable<AuthStore> = writable({
	token: null,
	user: null
});

/**
 * Updates the token in the auth store.
 *
 * @param {string | null | undefined} token - The new token to set. Use null to clear the token.
 */
export const setToken = (token: string | null | undefined) => {
	if (typeof token === 'undefined') {
		return;
	}
	const store = get(authStore);
	if (store.token != token) {
		authStore.update((state) => ({
			...state,
			token
		}));
	}
};

/**
 * Updates the user in the auth store.
 *
 * @param {User | null} user - The user to be updated in the auth store.
 */
export const setUser = (user: User | null) => {
	authStore.update((state) => ({
		...state,
		user
	}));
};

/**
 * Retrieves the token from the authStore.
 */
export const getToken = () => {
	return get(authStore).token;
};
