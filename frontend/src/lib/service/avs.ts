import { API_URL } from '$lib/const/urls';
import type { Avs, Message, Result } from '$lib/types/response';
import { del, get, patchJson } from './utils';

/**
 * Retrieves the list of Avs using the provided token.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<Avs[]>>} A promise that resolves to a Result object containing the list of Avs.
 */
export async function getAvs(token: string): Promise<Result<Avs[]>> {
	return await get(`${API_URL}/avs`, { api_key: token });
}

/**
 * Accepts an AVS using the provided token and ID.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the AVS to accept.
 * @return {Promise<Result<Message>>} A promise that resolves with the result of the acceptance operation.
 */
export async function acceptAvs(token: string, id: number): Promise<Result<Message>> {
	return await patchJson(`${API_URL}/avs/accept/${id}`, {}, { api_key: token });
}

/**
 * Updates a partial Avs record.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the Avs record.
 * @param {string} address - The new address for the Avs record.
 * @param {string} description - The new description for the Avs record.
 * @return {Promise<Result<Avs>>} A promise that resolves to the updated Avs record.
 */
export async function updatePartialAvs(
	token: string,
	id: number,
	address: string,
	description: string
): Promise<Result<Avs>> {
	return await patchJson(
		`${API_URL}/avs/${id}`,
		{ address, description, kind: null, lat: null, lng: null },
		{ api_key: token }
	);
}

/**
 * Deletes an AVS.
 *
 * @param {string} token - The access token.
 * @param {number} id - The ID of the AVS to delete.
 * @return {Promise<Result<Message>>} - A promise that resolves to a Result object containing a Message.
 */
export async function deleteAvs(token: string, id: number): Promise<Result<Message>> {
	return await del(`${API_URL}/avs/${id}`, { api_key: token });
}
