import { BACKEND_URL } from '$env/static/private';
import { get_control, get_control_completion } from '$lib/remote/controls';
import { getMeasuresForControl } from '$lib/remote/measures';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { fail, type Actions } from '@sveltejs/kit';

export async function load({ cookies, params }) {
	const id = params.id;
	let session = await getSessionFromCookiesOrCreate(cookies);
	let control = await get_control(session, id);
	let progress = await get_control_completion(session, id);

	return {
		control,
		measures: control ? await getMeasuresForControl(session, control.identifier) : [],
		progress
	};
}

export const actions: Actions = {
	edit_control: async ({ request, fetch, cookies }) => {
		let session = await getSessionFromCookiesOrCreate(cookies);
		const formData = await request.formData();

		const title = formData.get('title')?.toString();
		const description = formData.get('description')?.toString();
		const id = formData.get('id')?.toString();

		if (title == undefined) {
			return fail(400, { reason: 'missing title' });
		}
		if (description == undefined) {
			return fail(400, { reason: 'missing description' });
		}
		if (id == undefined) {
			return fail(400, { reason: 'missing data' });
		}

		const data = new FormData();
		data.set('title', title);
		data.set('description', description);

		const response = await fetch(BACKEND_URL + `/api/v1/controls/${id}`, {
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
