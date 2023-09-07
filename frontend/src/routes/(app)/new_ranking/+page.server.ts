import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions, redirect } from '@sveltejs/kit';

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();

		let session = await getSessionFromCookiesOrCreate(cookies);
		try {
			let min_coverage = Number.parseInt(formData.get('minimum_coverage')?.toString() || 'nope');
			if (min_coverage > 0 && min_coverage <= 100) {
				let data = new FormData();
				data.set('minimum_coverage', min_coverage.toString());
				if (formData.get('filter_by_tag') == 'on') {
					// Gather the tags
					let tag_ids = formData.getAll('tag') || [];
					for (let tag of tag_ids) {
						data.append('filter_tags', tag);
					}
					// Gather all or any
					data.set('all_tags', formData.get('all_tags') == 'on' ? 'true' : 'false');
				}
				const response = await fetch(BACKEND_URL + '/api/v1/rankings', {
					method: 'POST',
					headers: {
						Authorization: `Bearer ${session.auth_token}`
					},
					body: data
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
