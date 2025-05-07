import { verify } from '$lib/service/auth';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const uuid = params.uuid;
	const res = await verify(uuid);
	return res;
};
