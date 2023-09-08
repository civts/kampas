import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions } from '@sveltejs/kit';

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		let session = await getSessionFromCookiesOrCreate(cookies);

		let title = formData.get('title')?.toString().trim();
		let description = formData.get('description')?.toString().trim();
		let effort_str = formData.get('effort')?.toString().trim();
		let effort: number;
		if (title == undefined || title.length < 3) {
			return fail(400, { title_error: 'The title must be longer than 3 characters' });
		}
		if (description == undefined || description.length < 3) {
			return fail(400, { description_error: 'The description must be longer than 3 characters' });
		}

		if (effort_str == undefined) {
			return fail(400, { description_error: 'Specify an effort' });
		}
		try {
			effort = Number.parseInt(effort_str);
			if (effort < 1) {
				throw 'invalid';
			}
		} catch (error) {
			return fail(400, { effort_error: 'Specify an effort. It must be a positive integer' });
		}

		formData.set('title', title);
		formData.set('description', description);
		formData.set('effort', effort_str);

		const response = await fetch(BACKEND_URL + '/api/v1/metrics', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			},
			body: formData
		});
		if (response.ok) {
			return { success: true };
		} else {
			return fail(response.status, {
				reason: 'Could not create the metric. The server said:' + (await response.text())
			});
		}
	}
};
