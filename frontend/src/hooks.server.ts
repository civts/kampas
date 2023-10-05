import { redirect, type Handle, type RequestEvent } from '@sveltejs/kit';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { updateSession } from '$lib/session_redis';
import { BACKEND_URL } from '$env/static/private';

export const handle: Handle = async ({ event, resolve }) => {
	const session = await getSessionFromCookiesOrCreate(event.cookies);

	if (requiresAuthentication(event)) {
		console.log(event.url.pathname + ' does require authentication');
		const isAuthenticated = await checkAuthentication(session);

		if (!isAuthenticated) {
			console.log('Existing auth is invalid');
			throw redirect(302, '/login');
		}
	}

	return resolve(event);
};

function requiresAuthentication(event: RequestEvent): boolean {
	// Determine if the current route requires authentication
	// based on the event object or any other criteria
	console.log(event.url.pathname);
	const whitelist = ['/register', '/login'];
	let isWhitelisted = whitelist.indexOf(event.url.pathname) != -1;
	return !isWhitelisted;
}

async function checkAuthentication(session: Session): Promise<boolean> {
	try {
		const response = await fetch(BACKEND_URL + '/api/v1/auth/whoami', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		if (response.ok) {
			const username = await response.text();
			if (session.username != username) {
				session.username = username;
				await updateSession(session);
			}
			return true;
		} else {
			return false;
		}
	} catch (error) {
		console.error('Error checking authentication:', error);
		return false;
	}
}
