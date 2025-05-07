import { API_URL } from '$lib/const/urls';
import type { Result, Schedule } from '$lib/types/response';
import { del, get, putJson, postJson } from './utils';

/**
 * Creates a schedule.
 *
 * @param {string} token - The API token.
 * @param {number[]} dates - An array of dates for the schedule.
 * @param {number[]} days - An array of days for the schedule.
 * @param {number[]} device_ids - An array of device IDs for the schedule.
 * @param {number} kind - The kind of schedule.
 * @param {number | null} month - The month for the schedule.
 * @param {string} name - The name of the schedule.
 * @param {number} records_id - The records ID for the schedule.
 * @param {string[]} times - An array of times for the schedule.
 * @param {string[]} volumes - An array of volumes for the schedule.
 * @param {number[]} weeks - An array of weeks for the schedule.
 * @param {number | null} year - The year for the schedule.
 * @return {Promise<Result<Schedule>>} A Promise that resolves with the created schedule.
 */
export async function createSchedule(
	token: string,
	dates: number[],
	days: number[],
	device_ids: number[],
	kind: number,
	month: number | null,
	name: string,
	records_id: number,
	times: string[],
	volumes: string[],
	weeks: number[],
	year: number | null
): Promise<Result<Schedule>> {
	return await postJson(
		`${API_URL}/schedules`,
		{
			dates,
			days,
			device_ids,
			kind,
			month,
			name,
			records_id,
			times,
			volumes,
			weeks,
			year
		},
		{ api_key: token }
	);
}

/**
 * Retrieves the schedule using the provided token.
 *
 * @param {string} token - The authentication token.
 * @return {Promise<Result<Schedule[]>>} A Promise that resolves to a Result object containing an array of Schedule objects.
 */
export async function getSchedule(token: string): Promise<Result<Schedule[]>> {
	return get(`${API_URL}/schedules`, { api_key: token });
}

/**
 * Deletes a schedule with the specified ID.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the schedule to be deleted.
 * @return {Promise<Result<Schedule>>} A promise that resolves to the result of the deletion operation.
 */
export async function deleteSchedule(token: string, id: number): Promise<Result<Schedule>> {
	return del(`${API_URL}/schedules/${id}`, { api_key: token });
}

/**
 * Retrieves a schedule by its ID.
 *
 * @param {string} token - The API token for authentication.
 * @param {number} id - The ID of the schedule to retrieve.
 * @return {Promise<Result<Schedule>>} A Promise that resolves to the result of the request, containing the schedule data.
 */
export async function getScheduleById(token: string, id: number): Promise<Result<Schedule>> {
	return get(`${API_URL}/schedules/${id}`, { api_key: token });
}

/**
 * Updates the schedule volume for a given schedule ID.
 *
 * @param {string} token - The authentication token.
 * @param {number} id - The ID of the schedule.
 * @param {string[]} volumes - The array of volumes to update.
 * @return {Promise<Result<Schedule>>} A promise that resolves to a Result object containing the updated schedule.
 */
export async function updateScheduleVolume(
	token: string,
	id: number,
	volumes: string[]
): Promise<Result<Schedule>> {
	return await putJson(`${API_URL}/schedules/${id}`, { volumes }, { api_key: token });
}
