import { BACKEND_URL } from '$lib/costants';
import { get_control } from '$lib/remote/controls';
import { getMetricsForControl } from '$lib/remote/metrics';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions } from '@sveltejs/kit';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let control = await get_control(session, id);

	return {
		control,
		metrics: control ? await getMetricsForControl(session, control.identifier) : []
	};
}

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const formData = await request.formData();
		let session = await getSessionFromCookiesOrCreate(cookies);

		let title = formData.get('title')?.toString().trim();
		let description = formData.get('description')?.toString().trim();
		let control_id = formData.get('control_id')?.toString().trim();
		let effort_str = formData.get('effort')?.toString().trim();
		let coverage_str = formData.get('coverage')?.toString().trim();
		let coverage: number;
		let effort: number;
		if (title == undefined || title.length < 3) {
			return fail(400, { title_error: 'The title must be longer than 3 characters' });
		}
		if (description == undefined || description.length < 3) {
			return fail(400, { description_error: 'The description must be longer than 3 characters' });
		}
		if (control_id == undefined || control_id.length < 3) {
			return fail(400, {
				reason: `Hey ${session.username}, stop playing with the code :)`
			});
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
		if (coverage_str == undefined) {
			return fail(400, { description_error: 'Specify a coverage' });
		}
		try {
			coverage = Number.parseInt(coverage_str);
			if (coverage < 1 || coverage > 100) {
				throw 'invalid';
			}
		} catch (error) {
			return fail(400, {
				coverage_error: 'Specify a coverage. It must be an integer between 1 and 100'
			});
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
			const id = await response.text();

			let data = new FormData();
			data.set('metric_id', id);
			data.set('control_id', control_id);
			data.set('coverage', coverage_str);

			// Associating the new metric with this control
			const response2 = await fetch(BACKEND_URL + '/api/v1/metrics/associate', {
				method: 'POST',
				headers: {
					Authorization: `Bearer ${session.auth_token}`
				},
				body: data
			});

			if (response2.ok) {
				return { success: true };
			} else {
				return fail(response2.status, {
					reason: 'Could not associate the metric to the control.'
				});
			}
		} else {
			return fail(response.status, {
				reason: 'Could not create the metric. The server said:' + (await response.text())
			});
		}
	}
};
