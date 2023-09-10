import { get_control, get_control_completion } from '$lib/remote/controls';
import { getEnablersForControl } from '$lib/remote/enablers';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let control = await get_control(session, id);
	let progress = await get_control_completion(session, id);

	return {
		control,
		enablers: control ? await getEnablersForControl(session, control.identifier) : [],
		progress
	};
}
