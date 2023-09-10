import { BACKEND_URL } from '$env/static/private';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { RequestHandler } from '@sveltejs/kit';

const addTagToControl: RequestHandler = async ({ request, cookies }) => {
	const session = await getSessionFromCookiesOrCreate(cookies);
	const req_json = await request.json();

	let control_id = req_json['control_id'];
	let tag_id = req_json['tag_id'];
	console.log(tag_id);
	let data = new FormData();
	data.set('control_id', control_id);
	data.set('tag_id', tag_id);

	let endpoint = `${BACKEND_URL}/api/v1/tags/tag_control`;
	const response = await fetch(endpoint, {
		method: 'POST',
		headers: { Authorization: `Bearer ${session.auth_token}` },
		body: data
	});
	if (response.ok) {
		return new Response('Yess');
	} else {
		return new Response('Could not talk to server', { status: 500 });
	}
};

const removeTagFromControl: RequestHandler = async ({ request, cookies }) => {
	const session = await getSessionFromCookiesOrCreate(cookies);
	const req_json = await request.json();

	let control_id = req_json['control_id'];
	let tag_id = req_json['tag_id'];
	console.log(tag_id);
	let data = new FormData();
	data.set('control_id', control_id);
	data.set('tag_id', tag_id);

	let endpoint = `${BACKEND_URL}/api/v1/tags/tag_control`;
	const response = await fetch(endpoint, {
		method: 'DELETE',
		headers: { Authorization: `Bearer ${session.auth_token}` },
		body: data
	});
	if (response.ok) {
		return new Response('Yess');
	} else {
		return new Response('Could not talk to server', { status: 500 });
	}
};

export const POST = addTagToControl;
export const DELETE = removeTagFromControl;
