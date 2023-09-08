import { get_control } from '$lib/remote/controls';
import { getMetricsForControl } from '$lib/remote/metrics';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let control = await get_control(session, id);

	return {
		control,
		metrics: control ? await getMetricsForControl(session, control.identifier) : []
	};
}
