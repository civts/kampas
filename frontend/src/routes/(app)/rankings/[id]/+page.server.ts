import type { Control } from '$lib/models/bindings/Control';
import type { Measure } from '$lib/models/bindings/Measure';
import type { Tag } from '$lib/models/bindings/Tag';
import { get_controls } from '$lib/remote/controls';
import { getMeasure } from '$lib/remote/measures';
import { get_ranking } from '$lib/remote/ranking';
import { get_tags_for_control_batch, get_tags_for_measure } from '$lib/remote/tags';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let ranking = await get_ranking(session, id);

	let measures: Measure[] = [];
	let measures_tags = new Map<string, Tag[]>();
	for (let id of ranking?.measures || []) {
		let m = await getMeasure(session, id);
		if (m != undefined) {
			measures.push(m);
			measures_tags.set(id, await get_tags_for_measure(id, session));
		} else {
			console.error('It seems we requested an inexistent measure: ', id);
		}
	}

	let controls: Control[] = await get_controls(session);
	controls = controls.filter((c) => ranking?.controls.includes(c.identifier));
	let control_tags = await get_tags_for_control_batch(
		controls.map((c) => c.identifier),
		session
	);

	return {
		ranking,
		measures,
		measures_tags,
		controls,
		control_tags
	};
}
