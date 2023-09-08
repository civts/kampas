import { GTK_A11Y } from '$env/static/private';
import { get_rankings } from '$lib/remote/ranking';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';

export async function load({ cookies }) {
	let session = await getSessionFromCookiesOrCreate(cookies);
	let rankings = await get_rankings(session);

	rankings?.sort((a, b) => {
		let date_a = new Date(Number.parseInt(a.created_at));
		let date_b = new Date(Number.parseInt(b.created_at));
		if (date_a > date_b) {
			return -1;
		} else if (date_a < date_b) {
			return 1;
		} else {
			return 0;
		}
	});
	return {
		rankings
	};
}
