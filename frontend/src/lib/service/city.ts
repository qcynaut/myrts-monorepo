import { API_URL } from '$lib/const/urls';
import type { City, Result } from '$lib/types/response';
import { get } from './utils';

/**
 * Retrieves a list of cities from the API server.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<City[]>>} A promise that resolves to a Result object containing an array of City objects.
 */
export async function getCites(token: string): Promise<Result<City[]>> {
	return await get(`${API_URL}/cities`, { api_key: token });
}
