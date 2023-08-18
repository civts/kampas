import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { PageServerLoad } from './$types';

type Control = {
	title: String;
	description: String;
};

export const load: PageServerLoad = async ({ cookies }) => {
	let session = await getSessionFromCookiesOrCreate(cookies);

	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});
		let controls: Control[] = await controls_resp.json();
		console.log(controls);
		return { controls };
	} catch (error) {
		console.error('Error loading controls: ', error);
		return { controls: [] };
	}
};
