import { get_control, get_control_completion } from '$lib/remote/controls';
import { getMeasuresForControl } from '$lib/remote/measures';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let control = await get_control(session, id);
	let progress = await get_control_completion(session, id);

	return {
		control,
		measures: control ? await getMeasuresForControl(session, control.identifier) : [],
		progress
	};
}
