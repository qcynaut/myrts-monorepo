import { checkForgot } from '$lib/service/auth';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const uuid = params.uuid;
	const res = await checkForgot(uuid);
	return {
		error: res.error?.error,
		uuid
	};
};
