import { get_ranking } from '$lib/remote/controls';
import { getMetric } from '$lib/remote/metrics';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let ranking = await get_ranking(session, id);
	let metrics = [];

	for (let id of ranking?.metrics || []) {
		metrics.push(await getMetric(session, id));
	}

	return {
		ranking,
		metrics
	};
}
