import { BACKEND_URL } from '$env/static/private';
import type { Measure } from '$lib/models/bindings/Measure';

export async function getMeasuresForControl(
	session: Session,
	control_id: string
): Promise<Measure[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/measures?control_id=${control_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Measure[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function get_control_associated_to_measure_batch(
	session: Session,
	measure_ids: String[]
): Promise<number[]> {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/measures/get_number_controls_batch', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(measure_ids)
		});

		let completion: number[] = await controls_resp.json();
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return [];
	}
}

export async function getMeasures(session: Session): Promise<Measure[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/measures`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Measure[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function getMeasure(
	session: Session,
	measure_id: string
): Promise<Measure | undefined> {
	const response = await fetch(`${BACKEND_URL}/api/v1/measures/${measure_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Measure = await response.json();
		return data;
	} else {
		return undefined;
	}
}

export async function get_coverage_for_measure(measure_id: String, session: Session) {
	try {
		const coverage_resp = await fetch(
			BACKEND_URL + `/api/v1/measures/coverage_for_measure?measure_id=${measure_id}`,
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
