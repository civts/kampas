import type { Control } from '$lib/models/bindings/Control';
import type { Tag } from '$lib/models/bindings/Tag';
import { get_controls } from '$lib/remote/controls';
import { getMetric } from '$lib/remote/metrics';
import { get_ranking } from '$lib/remote/ranking';
import { get_tags_for_control_batch, get_tags_for_metric } from '$lib/remote/tags';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let ranking = await get_ranking(session, id);

	let metrics = [];
	let metrics_tags = new Map<string, Tag[]>();
	for (let id of ranking?.metrics || []) {
		metrics.push(await getMetric(session, id));
		metrics_tags.set(id, await get_tags_for_metric(id, session));
	}

	let controls: Control[] = await get_controls(session);
	controls = controls.filter((c) => ranking?.controls.includes(c.identifier));
	let control_tags = await get_tags_for_control_batch(
		controls.map((c) => c.identifier),
		session
	);

	return {
		ranking,
		metrics,
		metrics_tags,
		controls,
		control_tags
	};
}
