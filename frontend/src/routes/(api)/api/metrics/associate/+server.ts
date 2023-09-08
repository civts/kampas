import { BACKEND_URL } from '$lib/costants';
import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import type { RequestHandler } from '@sveltejs/kit';

const associateMetricToControl: RequestHandler = async ({ request, cookies }) => {
	const session = await getSessionFromCookiesOrCreate(cookies);
	const data = await request.json();

	const formData = new FormData();
	formData.set('metric_id', data.metric_id);
	formData.set('control_id', data.control_id);
	formData.set('coverage', data.coverage);

	const response = await fetch(`${BACKEND_URL}/api/v1/metrics/associate`, {
		method: 'POST',
		headers: { Authorization: `Bearer ${session.auth_token}` },
		body: formData
	});
	if (response.ok) {
		return new Response('ok');
	} else {
		return new Response('Could not talk to server', { status: 500 });
	}
};

export const POST = associateMetricToControl;
