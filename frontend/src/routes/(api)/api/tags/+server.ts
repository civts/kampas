import { BACKEND_URL } from '$env/static/private';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { RequestHandler } from '@sveltejs/kit';

const getTags: RequestHandler = async ({ url, cookies }) => {
	const session = await getSessionFromCookiesOrCreate(cookies);
	const control_id = url.searchParams.get('control_id');

	let endpoint;
	if (control_id == undefined) {
		// We fetch all the tags
		endpoint = `${BACKEND_URL}/api/v1/tags`;
	} else {
		// We fetch only the tags for the control we care about
		endpoint = `${BACKEND_URL}/api/v1/tags?control_id=${control_id}`;
	}
	const response = await fetch(endpoint, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data = await response.json();
		return new Response(JSON.stringify(data));
	} else {
		return new Response('Could not talk to server', { status: 500 });
	}
};

export const GET = getTags;
