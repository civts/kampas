import { get_control } from '$lib/remote/controls';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);

	return {
		control: await get_control(session, id)
	};
}
