import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$env/static/private';
import { fail, type Actions } from '@sveltejs/kit';
import { get_tags } from '$lib/remote/tags';

export const load: PageServerLoad = async ({ cookies }) => {
	let session = await getSessionFromCookiesOrCreate(cookies);

	return { tags: await get_tags(session) };
};

export const actions: Actions = {
	add_tag: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		let session = await getSessionFromCookiesOrCreate(cookies);

		const response = await fetch(BACKEND_URL + '/api/v1/tags', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			},
			body: formData
		});
		if (response.ok) {
			return { success: true };
		} else {
			return fail(response.status, { reason: 'The server said:' + (await response.text()) });
		}
	}
};
