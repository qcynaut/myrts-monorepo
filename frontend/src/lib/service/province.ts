import { API_URL } from '$lib/const/urls';
import type { IdName, Result } from '$lib/types/response';
import { get } from './utils';

/**
 * Retrieves a list of provinces from the API.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<IdName[]>>} A promise that resolves to a Result object containing an array of IdName objects.
 */
export async function getProvinces(token: string): Promise<Result<IdName[]>> {
	return await get(`${API_URL}/provinces`, { api_key: token });
}
