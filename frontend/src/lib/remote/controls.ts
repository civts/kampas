import { BACKEND_URL } from '$lib/costants';

export async function get_controls(session: Session) {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});
		console.log(controls_resp.status);

		let controls: Control[] = await controls_resp.json();
		return controls;
	} catch (error) {
		console.error('Error loading controls: ', error);
		return [];
	}
}
