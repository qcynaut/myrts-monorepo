import type { ApiError, Result } from '$lib/types/response';

/**
 * Represents the options for the API.
 */
interface ApiOpt {
	/**
	 * The API key to be used for authentication.
	 */
	api_key?: string;

	/**
	 * The content type of the API request.
	 */
	content_type?: string;
}

/**
 * Retrieves data from the specified URL.
 *
 * @param {string} url - The URL to fetch data from.
 * @return {Promise<Result<T>>} A Promise that resolves to a Result object containing the retrieved data or an error.
 */
export async function get<T>(url: string, opt?: ApiOpt): Promise<Result<T>> {
	try {
		const res = await fetch(url, {
			headers: {
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'application/json'
			}
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: T = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a POST request with JSON data to the specified URL.
 *
 * @param {string} url - The URL to send the request to.
 * @param {P} body - The JSON data to send in the request body.
 * @param {ApiOpt} [opt] - Additional options for the API request.
 * @returns {Promise<Result<T>>} - A promise that resolves to the result of the request.
 */
export async function postJson<P, T>(url: string, body: P, opt?: ApiOpt): Promise<Result<T>> {
	try {
		const res = await fetch(url, {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'application/json'
			},
			body: JSON.stringify(body)
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: T = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a DELETE request to the specified URL and returns the result.
 *
 * @param {string} url - The URL to send the DELETE request to.
 * @param {ApiOpt} [opt] - Optional parameters for the API request.
 * @returns {Promise<Result<T>>} A Promise that resolves to the result of the DELETE request.
 */
export async function del<T>(url: string, opt?: ApiOpt): Promise<Result<T>> {
	try {
		const res = await fetch(url, {
			method: 'DELETE',
			headers: {
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'application/json'
			}
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: T = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a PATCH request to the specified URL with the given body as JSON.
 *
 * @param {string} url - The URL to send the request to.
 * @param {P} body - The body of the request, in JSON format.
 * @param {ApiOpt} [opt] - Optional options object for the request.
 * @returns {Promise<Result<T>>} A Promise that resolves to the result of the request.
 */
export async function patchJson<P, T>(url: string, body: P, opt?: ApiOpt): Promise<Result<T>> {
	try {
		const res = await fetch(url, {
			method: 'PATCH',
			headers: {
				'Content-Type': 'application/json',
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'application/json'
			},
			body: JSON.stringify(body)
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: T = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a PUT request to the specified URL with the given body as JSON.
 *
 * @param {string} url - The URL to send the request to.
 * @param {P} body - The body of the request, in JSON format.
 * @param {ApiOpt} [opt] - Optional options object for the request.
 * @returns {Promise<Result<T>>} A Promise that resolves to the result of the request.
 */
export async function putJson<P, T>(url: string, body: P, opt?: ApiOpt): Promise<Result<T>> {
	try {
		const res = await fetch(url, {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json',
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'application/json'
			},
			body: JSON.stringify(body)
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: T = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a POST request to the specified URL with the provided form data.
 *
 * @param {string} url - The URL to send the request to.
 * @param {FormData} formData - The form data to send in the request body.
 * @param {ApiOpt} [opt] - Additional API options.
 * @returns {Promise<Result<any>>} A promise that resolves to the result of the request.
 */
export async function postForm(
	url: string,
	formData: FormData,
	opt?: ApiOpt
): Promise<Result<any>> {
	try {
		const res = await fetch(url, {
			method: 'POST',
			headers: {
				api_key: opt?.api_key || '',
				content_type: opt?.content_type || 'multipart/form-data'
			},
			body: formData
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: any = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}

/**
 * Sends a PATCH request to the specified URL with the given form data.
 *
 * @param {string} url - The URL to send the PATCH request to.
 * @param {FormData} formData - The form data to send in the request body.
 * @param {ApiOpt} [opt] - Optional additional options for the API request.
 * @returns {Promise<Result<any>>} A Promise that resolves to the result of the API request.
 */
export async function patchForm(
	url: string,
	formData: FormData,
	opt?: ApiOpt
): Promise<Result<any>> {
	try {
		const res = await fetch(url, {
			method: 'PATCH',
			headers: {
				api_key: opt?.api_key || ''
			},
			body: formData
		});
		const status = res.status;
		if (status !== 200) {
			const error: ApiError = await res.json();
			return {
				result: null,
				error,
				status
			};
		} else {
			const data: any = await res.json();
			return {
				result: data,
				status
			};
		}
	} catch (error) {
		return {
			result: null,
			error: { error: (error as Error).message },
			status: 500
		};
	}
}
