import type { Tag } from '$lib/models/bindings/Tag';
import {
	get_controls_for_metric,
	get_coverage_for_metric,
	get_tags_for_metric
} from '$lib/remote/controls';
import { getMetric } from '$lib/remote/metrics';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let metric = await getMetric(session, id);
	let controls = await get_controls_for_metric(id, session);
	let coverage = await get_coverage_for_metric(id, session);
	let tags = await get_tags_for_metric(id, session);

	return {
		metric,
		controls,
		coverage,
		tags
	};
}
