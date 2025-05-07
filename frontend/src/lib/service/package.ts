import { API_URL } from '$lib/const/urls';
import type { Package, Result } from '$lib/types/response';
import { get } from './utils';

/**
 * Retrieves packages using an API token.
 *
 * @param {string} token - The API token used to authenticate the request.
 * @return {Promise<Result<Package[]>>} A promise that resolves to a Result object containing an array of Package objects.
 */
export async function getPackages(token: string): Promise<Result<Package[]>> {
	return await get(`${API_URL}/packages`, { api_key: token });
}
