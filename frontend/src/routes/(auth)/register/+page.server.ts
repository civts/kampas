import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$env/static/private';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { updateSession } from '$lib/session_redis';

export const actions: Actions = {
	default: async ({ request, fetch, cookies }) => {
		const data = await request.formData();

		const username_pattern: string = '^[a-z0-9][a-z0-9_]{1,28}[a-z0-9]$';
		const password_pattern: string = '^.{12,256}$';

		const username = data.get('username')?.valueOf() as string | undefined;
		const password = data.get('password')?.valueOf() as string | undefined;
		const passwordConf = data.get('password_conf')?.valueOf() as string | undefined;

		// Perform validation
		if (username == undefined || !new RegExp(username_pattern).test(username)) {
			return fail(400, {
				username_error:
					'Invalid username. It must be between 3 and 30 alphanumeric characters (or underscores), all lowercase. It cannot start or end with an underscore.'
			});
		}

		if (password == undefined || password !== passwordConf) {
			return fail(400, { password_conf_error: 'Password confirmation does not match.' });
		}

		if (password == undefined || !new RegExp(password_pattern).test(password)) {
			return fail(400, {
				password_error: 'Invalid password. It must be between 12 and 256 characters.'
			});
		}

		// Proceed with submission
		let formData = new FormData();
		formData.append('username', username);
		formData.append('password', password);
		const response = await fetch(BACKEND_URL + '/api/v1/auth/register', {
			method: 'POST',
			body: formData
		});

		if (response.status == 200) {
			const login_response = await fetch(BACKEND_URL + '/api/v1/auth/generate_token', {
				method: 'POST',
				body: formData
			});

			if (login_response.status == 200) {
				const token = await login_response.text();

				//Get session
				const session = await getSessionFromCookiesOrCreate(cookies);

				//Set token
				session.auth_token = token;
				await updateSession(session);

				console.log(session);
				throw redirect(302, '/');
			} else {
				return fail(400, { reason: await login_response.text() });
			}
		} else {
			return fail(400, { reason: 'Registering failed: ' + (await response.text()) });
		}
	}
};
