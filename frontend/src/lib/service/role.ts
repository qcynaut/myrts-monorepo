import { API_URL } from '$lib/const/urls';
import type { IdName, Result } from '$lib/types/response';
import { get } from './utils';

/**
 * Retrieves the role information from the server using the provided token.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<IdName>>} A promise that resolves to the role information.
 */
export async function getRole(token: string): Promise<Result<IdName>> {
	return await get(`${API_URL}/roles`, { api_key: token });
}
