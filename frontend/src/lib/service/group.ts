import { API_URL } from '$lib/const/urls';
import type { Group, GroupData, Paginated, Result } from '$lib/types/response';
import { get, patchJson, postJson } from './utils';

/**
 * Retrieve paginated group data from the API.
 *
 * @param {number} page - The page number to retrieve.
 * @param {string} token - The API token for authentication.
 * @return {Promise<Result<Paginated<GroupData>>>} A Promise that resolves to a Result object containing the paginated group data.
 */
export async function get_group_paginated(
	page: number,
	token: string
): Promise<Result<Paginated<GroupData>>> {
	return await get(`${API_URL}/groups/paginated?page=${page}`, { api_key: token });
}

/**
 * Retrieves a list of groups from the server using the provided token.
 *
 * @param {string} token - The API token used for authentication.
 * @return {Promise<Result<Group[]>>} A promise that resolves to a Result object containing an array of Group objects.
 */
export async function getGroups(token: string): Promise<Result<Group[]>> {
	return await get(`${API_URL}/groups`, { api_key: token });
}

/**
 * Creates a new group.
 *
 * @param {string} token - The API token.
 * @param {string} name - The name of the group.
 * @param {string | null} description - The description of the group (optional).
 * @param {number | null} parent_id - The ID of the parent group (optional).
 * @returns {Promise<Result<Group>>} A Promise that resolves to the result of the group creation.
 */
export async function createGroup(token: string, name: string, description: string|null, parent_id: number|null): Promise<Result<Group>> {
	return await postJson(`${API_URL}/groups`, {name, description, parent_id}, { api_key: token });
}

/**
 * Updates a group in the system.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the group to update.
 * @param {string} name - The new name for the group.
 * @param {string|null} description - The new description for the group (or null if not provided).
 * @param {number|null} parent_id - The new parent ID for the group (or null if not provided).
 * @return {Promise<Result<Group>>} A Promise that resolves to the updated group object.
 */
export async function updateGroup(token: string, id: number, name: string, description: string|null, parent_id: number|null): Promise<Result<Group>> {
	return await patchJson(`${API_URL}/groups`, {id, name, description, parent_id }, { api_key: token });
}