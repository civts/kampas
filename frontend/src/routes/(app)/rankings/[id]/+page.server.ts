import type { Control } from '$lib/models/bindings/Control';
import type { Tag } from '$lib/models/bindings/Tag';
import { get_controls } from '$lib/remote/controls';
import { getEnabler } from '$lib/remote/enablers';
import { get_ranking } from '$lib/remote/ranking';
import { get_tags_for_control_batch, get_tags_for_enabler } from '$lib/remote/tags';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let ranking = await get_ranking(session, id);

	let enablers = [];
	let enablers_tags = new Map<string, Tag[]>();
	for (let id of ranking?.enablers || []) {
		enablers.push(await getEnabler(session, id));
		enablers_tags.set(id, await get_tags_for_enabler(id, session));
	}

	let controls: Control[] = await get_controls(session);
	controls = controls.filter((c) => ranking?.controls.includes(c.identifier));
	let control_tags = await get_tags_for_control_batch(
		controls.map((c) => c.identifier),
		session
	);

	return {
		ranking,
		enablers,
		enablers_tags,
		controls,
		control_tags
	};
}
