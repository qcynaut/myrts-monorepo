import { API_URL } from '$lib/const/urls';
import type { GroupData, Message, Result, StatisticsRes, User } from '$lib/types/response';
import { get_group_paginated } from './group';
import { del, get, patchForm, patchJson, postJson } from './utils';

/**
 * Retrieves the current user information from the API.
 *
 * @param {string} token - The API token for authentication.
 * @return {Promise<Result<User>>} - A promise that resolves to the user information.
 */
export async function current(token: string): Promise<Result<User>> {
	return await get(`${API_URL}/users/current`, { api_key: token });
}

/**
 * Retrieves the statistics from the API.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<StatisticsRes>>} A promise that resolves to the statistics response.
 */
export async function getStatistics(token: string): Promise<Result<StatisticsRes>> {
	return await get(`${API_URL}/statistics`, { api_key: token });
}

/**
 * Creates a new user with the provided information.
 *
 * @param {string} token - The user's authentication token.
 * @param {string} name - The name of the user.
 * @param {string} email - The email address of the user.
 * @param {number} role - The role of the user.
 * @param {number|null} package_id - The package ID of the user.
 * @param {number|null} city_id - The city ID of the user.
 * @param {number[]} devices - The list of devices associated with the user.
 * @param {number[]} group_ids - The list of group IDs associated with the user.
 * @return {Promise<Result<User>>} A promise that resolves to the result of the user creation.
 */
export async function createUser(
	token: string,
	name: string,
	email: string,
	role: number,
	package_id: number | null,
	city_id: number | null,
	devices: number[],
	group_ids: number[]
): Promise<Result<User>> {
	return await postJson(
		`${API_URL}/users`,
		{
			name,
			email,
			role,
			package_id,
			city_id,
			devices,
			group_ids
		},
		{ api_key: token }
	);
}

/**
 * Updates a user with the provided information.
 *
 * @param {string} token - The authentication token of the user.
 * @param {string} name - The new name of the user.
 * @param {string} email - The new email of the user.
 * @param {number} role - The new role of the user.
 * @param {number | null} city_id - The new city ID of the user.
 * @param {number[]} devices - The new device IDs of the user.
 * @param {number[]} group_ids - The new group IDs of the user.
 * @param {number} id - The ID of the user to be updated.
 * @return {Promise<Result<User>>} A promise that resolves to the updated user.
 */
export async function updateUser(
	token: string,
	name: string,
	email: string,
	role: number,
	city_id: number | null,
	devices: number[],
	group_ids: number[],
	id: number
): Promise<Result<User>> {
	return await patchJson(
		`${API_URL}/users`,
		{
			name,
			email,
			role_id: role,
			city_id,
			device_ids: devices,
			user_group_ids: group_ids,
			id
		},
		{ api_key: token }
	);
}

/**
 * Deletes a user from the API.
 *
 * @param {string} token - The token for authentication.
 * @param {number} id - The ID of the user to delete.
 * @return {Promise<Result<User>>} A promise that resolves to the result of the deletion operation.
 */
export async function deleteUser(token: string, id: number): Promise<Result<User>> {
	return await del(`${API_URL}/users/${id}`, { api_key: token });
}

/**
 * Patch an image file for a user.
 *
 * @param {string} token - The user's authentication token.
 * @param {File} image - The image file to be patched.
 * @return {Promise<Result<User>>} - A promise that resolves to a result containing the updated user.
 */
export async function patchImage(token: string, image: File): Promise<Result<User>> {
	const form = new FormData();
	form.append('file', image);
	return await patchForm(`${API_URL}/users/image`, form, { api_key: token });
}

const getNestedUser = (groups: GroupData[]) => {
	let users: User[] = [];
	for (const group of groups) {
		users = [...users, ...group.users];
		if (group.children.length > 0) {
			users = [...users, ...getNestedUser(group.children)];
		}
	}
	return users;
};

export async function getUsers(token: string): Promise<User[]> {
	let users: User[] = [];
	let total_page = 1;
	let current_page = 1;
	const res = await get_group_paginated(1, token);
	if (res.result?.items) {
		total_page = res.result.total_page;
		for (const item of res.result.items) {
			users = [...users, ...item.users];
			users = [...users, ...getNestedUser(item.children)];
		}
		while (current_page < total_page) {
			current_page++;
			const res = await get_group_paginated(current_page, token);
			if (res.result?.items) {
				total_page = res.result.total_page;
				for (const item of res.result.items) {
					users = [...users, ...item.users];
					users = [...users, ...getNestedUser(item.children)];
				}
			}
		}
	}

	return users;
}
