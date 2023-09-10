import { BACKEND_URL } from '$env/static/private';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { RequestHandler } from '@sveltejs/kit';

const getMeasures: RequestHandler = async ({ cookies }) => {
	const session = await getSessionFromCookiesOrCreate(cookies);
	const response = await fetch(`${BACKEND_URL}/api/v1/measures`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data = await response.json();
		return new Response(JSON.stringify(data));
	} else {
		return new Response('Could not talk to server', { status: 500 });
	}
};

export const GET = getMeasures;
