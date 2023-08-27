import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { PageServerLoad } from './$types';
import { get_controls } from '$lib/remote/controls';

export const load: PageServerLoad = async ({ cookies }) => {
	let session = await getSessionFromCookiesOrCreate(cookies);

	return { controls: await get_controls(session) };
};
