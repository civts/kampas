import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { PageServerLoad } from './$types';
import { get_controls } from '$lib/remote/controls';
import { get_tags_for_control_batch } from '$lib/remote/tags';

export const load: PageServerLoad = async ({ cookies }) => {
	let session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);
	const tags = await get_tags_for_control_batch(
		controls.map((c) => c.identifier),
		session
	);

	return { controls, tags };
};
