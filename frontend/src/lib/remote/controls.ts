import { BACKEND_URL } from '$lib/costants';

export async function get_controls(session: Session) {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let controls: Control[] = await controls_resp.json();
		return controls;
	} catch (error) {
		console.error('Error loading controls: ', error);
		return [];
	}
}

export async function get_control(session: Session, id: String) {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls/' + id, {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let controls: Control = await controls_resp.json();
		return controls;
	} catch (error) {
		console.error('Error loading control: ', error);
		return undefined;
	}
}
