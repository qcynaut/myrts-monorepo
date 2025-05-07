import { API_URL } from '$lib/const/urls';
import type { AuthRes, Message, Result, VerifyRes } from '$lib/types/response';
import { del, get, postJson } from './utils';

/**
 * Checks the validity of a given token by making a request to the API's check endpoint.
 *
 * @param {string} token - The token to be checked.
 * @return {Promise<Result<Message>>} - A promise that resolves to a result object containing a message.
 */
export async function check(token: string): Promise<Result<Message>> {
	return await get(`${API_URL}/auth`, { api_key: token });
}

/**
 * Sends a forgot password request to the server.
 *
 * @param {string} email - The email of the user requesting a password reset.
 * @return {Promise<Result<Message>>} A promise that resolves to a string or null.
 */
export async function forgot(email: string): Promise<Result<Message>> {
	return await postJson(`${API_URL}/auth/forgot`, { email });
}

/**
 * Checks if the provided UUID is valid for resetting the password.
 *
 * @param {string} uuid - The UUID for password reset.
 * @return {Promise<Result<Message>>} Returns a Promise that resolves to a string or null.
 */
export async function checkForgot(uuid: string): Promise<Result<Message>> {
	return await get(`${API_URL}/auth/reset/${uuid}`);
}

/**
 * Resets the password for a user.
 *
 * @param {string} uuid - The unique identifier for the password reset.
 * @param {string} password - The new password to set.
 * @returns {Promise<Result<Message>>} - A promise that resolves to either a string error message or null.
 */
export async function resetPassword(uuid: string, password: string): Promise<Result<Message>> {
	return await postJson(`${API_URL}/auth/reset/${uuid}`, { password });
}

/**
 * Verifies the UUID by sending a GET request to the authentication verification endpoint.
 *
 * @param {string} uuid - The UUID to be verified.
 * @return {Promise<Result<VerifyRes>>} - A Promise that resolves to the result of the verification.
 */
export async function verify(uuid: string): Promise<Result<VerifyRes>> {
	return await get(`${API_URL}/auth/verify/${uuid}`);
}

/**
 * Authenticates the user with the provided email and password.
 *
 * @param {string} email - The email of the user.
 * @param {string} password - The password of the user.
 * @return {Promise<Result<AuthRes>>} - A promise that resolves to the authentication response.
 */
export async function auth(email: string, password: string): Promise<Result<AuthRes>> {
	return await postJson(`${API_URL}/auth`, { email, password, mobile: false });
}

/**
 * Logout the user by deleting the authentication token.
 *
 * @param {string} token - The authentication token of the user.
 * @return {Promise<Result<Message>>} A promise that resolves to a Result object containing a Message.
 */
export async function logout(token: string): Promise<Result<Message>> {
	return await del(`${API_URL}/auth`, { api_key: token });
}
