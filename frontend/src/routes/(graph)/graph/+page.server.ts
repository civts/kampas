import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { get_controls, get_measure_control_association_batch } from '$lib/remote/controls';
import { getMeasures } from '$lib/remote/measures';

export async function load({ cookies }) {
	const session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);

	const measures = await getMeasures(session);
	measures.sort((a, b) => {
		if (a.title.toLocaleLowerCase() > b.title.toLocaleLowerCase()) {
			return 1;
		}
		return -1;
	});

	const links = await get_measure_control_association_batch(session);

	return {
		controls,
		measures,
		links
	};
}
