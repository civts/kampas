import { get_ranking } from '$lib/remote/controls';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let ranking = await get_ranking(session, id);

	return {
		ranking
	};
}
