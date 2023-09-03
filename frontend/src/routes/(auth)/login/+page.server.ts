import { fail, redirect } from '@sveltejs/kit';
import type { Actions } from './$types';
import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { updateSession } from '$lib/session_redis';

export const actions: Actions = {
	default: async ({ request, fetch, cookies }) => {
		const data = await request.formData();

		const username = data.get('username')?.valueOf() as string | undefined;
		const password = data.get('password')?.valueOf() as string | undefined;

		// Perform validation
		if (username == undefined) {
			return fail(400, {
				username_error: 'No username provided.'
			});
		}

		if (password == undefined) {
			return fail(400, { password_error: 'No password provided.' });
		}

		// Proceed with submission
		let formData = new FormData();
		formData.append('username', username);
		formData.append('password', password);
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
			return fail(400, {
				reason: 'Invalid credentials. Check that the username and password are correct'
			});
		}
	}
};
