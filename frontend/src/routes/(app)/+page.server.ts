import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { get_controls } from '$lib/remote/controls';

export async function load({ cookies }) {
	const session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);

	return {
		controls
	};
}
