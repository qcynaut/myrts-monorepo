import { API_URL } from '$lib/const/urls';
import type { Records, Result } from '$lib/types/response';
import { del, get, patchJson, postForm } from './utils';

/**
 * Creates a new record.
 *
 * @param {string} token - The authentication token.
 * @param {string} name - The name of the record.
 * @param {string | null} description - The description of the record (optional).
 * @param {number[]} user_ids - An array of user IDs associated with the record.
 * @param {File} file - The file to be uploaded.
 * @returns {Promise<Result<Records>>} - A Promise that resolves to a Result object containing the created record.
 */
export async function createRecord(
	token: string,
	name: string,
	description: string | null,
	user_ids: number[],
	file: File
): Promise<Result<Records>> {
	const formData = new FormData();
	formData.append('name', name);
	if (description) {
		formData.append('description', description);
	}
	for (const user of user_ids) {
		formData.append('user_ids[]', user.toString());
	}
	formData.append('file', file);
	return await postForm(`${API_URL}/records`, formData, { api_key: token });
}

/**
 * Retrieves records using the provided token.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<Records[]>>} A promise that resolves to a result containing an array of records.
 */
export async function getRecords(token: string): Promise<Result<Records[]>> {
	return await get(`${API_URL}/records`, { api_key: token });
}

/**
 * Deletes records with the specified ID.
 *
 * @param {string} token - The API token.
 * @param {number} id - The ID of the record to delete.
 * @return {Promise<Result<Records>>} A promise that resolves with the result of the deletion operation.
 */
export async function deleteRecords(token: string, id: number): Promise<Result<Records>> {
	return await del(`${API_URL}/records/${id}`, { api_key: token });
}

/**
 * Accepts a record by sending a PATCH request to the API with the provided token and record ID.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the record to accept.
 * @return {Promise<Result<Records>>} A Promise that resolves to the updated records.
 */
export async function acceptRecord(token: string, id: number): Promise<Result<Records>> {
	return await patchJson(`${API_URL}/records/status/${id}`, { val: 1 }, { api_key: token });
}
