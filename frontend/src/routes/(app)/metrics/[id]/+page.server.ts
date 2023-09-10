import { BACKEND_URL } from '$env/static/private';
import { get_controls_for_metric } from '$lib/remote/controls';
import { getMetric, get_coverage_for_metric } from '$lib/remote/metrics';
import { get_tags_for_metric } from '$lib/remote/tags';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions } from '@sveltejs/kit';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let metric = await getMetric(session, id);
	let controls = await get_controls_for_metric(id, session);
	let coverage = await get_coverage_for_metric(id, session);
	let tags = await get_tags_for_metric(id, session);

	return {
		metric,
		controls,
		coverage,
		tags
	};
}

export const actions: Actions = {
	edit_metric: async ({ request, fetch, cookies }) => {
		let session = await getSessionFromCookiesOrCreate(cookies);
		const formData = await request.formData();

		const title = formData.get('title')?.toString();
		const description = formData.get('description')?.toString();
		const effort = formData.get('effort')?.toString();
		const progress = formData.get('progress')?.toString();
		const id = formData.get('id')?.toString();

		if (title == undefined) {
			return fail(400, { reason: 'missing title' });
		}
		if (description == undefined) {
			return fail(400, { reason: 'missing description' });
		}
		if (effort == undefined) {
			return fail(400, { reason: 'missing effort' });
		}
		if (progress == undefined) {
			return fail(400, { reason: 'missing progress' });
		}
		if (id == undefined) {
			return fail(400, { reason: 'missing data' });
		}

		try {
			const e = Number.parseInt(effort);
			if (e < 0) {
				throw 'invalid';
			}
		} catch (error) {
			return fail(400, { reason: 'Invalid effort' });
		}

		try {
			const p = Number.parseInt(progress);
			if (p < 0 || p > 100) {
				throw 'invalid';
			}
		} catch (error) {
			return fail(400, { reason: 'Invalid progress' });
		}

		const data = new FormData();
		data.set('title', title);
		data.set('description', description);
		data.set('effort', effort);
		data.set('progress', progress);

		const response = await fetch(BACKEND_URL + `/api/v1/metrics/${id}`, {
			method: 'PATCH',
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			},
			body: data
		});
		if (response.ok) {
			return { success: true };
		} else {
			return fail(response.status, { reason: 'The server said:' + (await response.text()) });
		}
	}
};
