import { BACKEND_URL } from '$env/static/private';
import type { Control } from '$lib/models/bindings/Control';

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

export async function get_controls_for_measure(measure_id: String, session: Session) {
	try {
		const controls_resp = await fetch(BACKEND_URL + `/api/v1/controls?measure_id=${measure_id}`, {
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

export async function get_control_completion(
	session: Session,
	control_id: String
): Promise<number> {
	try {
		const controls_resp = await fetch(
			BACKEND_URL + '/api/v1/controls/get_completion?control_id=' + control_id,
			{
				headers: {
					Authorization: `Bearer ${session.auth_token}`
				}
			}
		);

		let completion: number = await controls_resp.json();
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return 0;
	}
}

export async function get_control_completion_batch(
	session: Session,
	control_ids: String[]
): Promise<number[]> {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls/get_completion_batch', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(control_ids)
		});

		let completion: number[] = await controls_resp.json();
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return [];
	}
}

export async function get_number_of_measures_per_control_batch(
	session: Session
): Promise<Map<string, number>> {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/controls/get_measures_count_batch', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let completion: Map<string, number> = new Map(Object.entries(await controls_resp.json()));
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return new Map();
	}
}

export async function get_measure_control_association_batch(
	session: Session
): Promise<Map<string, [String, Number][]>> {
	try {
		const controls_resp = await fetch(
			BACKEND_URL + '/api/v1/controls/get_measure_control_association_batch',
			{
				headers: {
					Authorization: `Bearer ${session.auth_token}`
				}
			}
		);

		let completion: Map<string, [String, Number][]> = new Map(
			Object.entries(await controls_resp.json())
		);
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return new Map();
	}
}
