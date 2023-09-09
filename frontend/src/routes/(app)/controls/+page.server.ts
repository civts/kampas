import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { PageServerLoad } from './$types';
import { get_controls } from '$lib/remote/controls';
import { get_tags_for_control_batch } from '$lib/remote/tags';
import type { Tag } from '$lib/models/bindings/Tag';

export const load: PageServerLoad = async ({ cookies }) => {
	let session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);
	const tags_resp = await get_tags_for_control_batch(
		controls.map((c) => c.identifier),
		session
	);

	const tags: Map<string, Tag[]> = new Map(await tags_resp.json());
	return { controls, tags };
};
