import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		let session = await getSessionFromCookiesOrCreate(cookies);

		const response = await fetch(BACKEND_URL + '/api/v1/controls', {
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
