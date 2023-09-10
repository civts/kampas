import { BACKEND_URL } from '$env/static/private';
import type { Enabler } from '$lib/models/bindings/Enabler';

export async function getEnablersForControl(
	session: Session,
	control_id: string
): Promise<Enabler[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/enablers?control_id=${control_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Enabler[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function get_control_associated_to_enabler_batch(
	session: Session,
	enabler_ids: String[]
): Promise<number[]> {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/enablers/get_number_controls_batch', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(enabler_ids)
		});

		let completion: number[] = await controls_resp.json();
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return [];
	}
}

export async function getEnablers(session: Session): Promise<Enabler[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/enablers`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Enabler[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function getEnabler(
	session: Session,
	enabler_id: string
): Promise<Enabler | undefined> {
	const response = await fetch(`${BACKEND_URL}/api/v1/enablers/${enabler_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Enabler = await response.json();
		return data;
	} else {
		return undefined;
	}
}

export async function get_coverage_for_enabler(enabler_id: String, session: Session) {
	try {
		const coverage_resp = await fetch(
			BACKEND_URL + `/api/v1/enablers/coverage_for_enabler?enabler_id=${enabler_id}`,
			{
				headers: {
					Authorization: `Bearer ${session.auth_token}`
				}
			}
		);
		const coverage = new Map(Object.entries(await coverage_resp.json()));
		return coverage;
	} catch (error) {
		console.error('Error loading coverage: ', error);
		return new Map<String, number>();
	}
}
