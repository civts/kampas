import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies }) {
	let session = await getSessionFromCookiesOrCreate(cookies);

	return {
		user: session.username
	};
}
