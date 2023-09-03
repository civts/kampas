import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		let session = await getSessionFromCookiesOrCreate(cookies);
		try {
			let data = Number.parseInt(formData.get('minimum_coverage')?.toString() || 'nope');
			if (data > 0 && data <= 100) {
				const response = await fetch(BACKEND_URL + '/api/v1/rankings', {
					method: 'POST',
					headers: {
						Authorization: `Bearer ${session.auth_token}`
					},
					body: formData
				});
				if (response.ok) {
					return { success: true, ranking_id: await response.text() };
				}
				return fail(response.status, { reason: 'The server responded ' + response.status });
			}
			return fail(400, { reason: 'The minimum coverage must be between 1 and 100 (inclusive)' });
		} catch (e) {
			return fail(400, { reason: 'The minimum coverage must be between 1 and 100 (inclusive)' });
		}
	}
};
