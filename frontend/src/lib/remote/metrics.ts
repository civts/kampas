import { BACKEND_URL } from '$lib/costants';
import type { Metric } from '$lib/models/bindings/Metric';

export async function getMetricsForControl(
	session: Session,
	control_id: string
): Promise<Metric[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/metrics?control_id=${control_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Metric[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function get_control_associated_to_metric_batch(
	session: Session,
	metric_ids: String[]
): Promise<number[]> {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/metrics/get_number_controls_batch', {
			method: 'POST',
			headers: {
				Authorization: `Bearer ${session.auth_token}`,
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(metric_ids)
		});

		let completion: number[] = await controls_resp.json();
		return completion;
	} catch (error) {
		console.error('Error loading completion for control: ', error);
		return [];
	}
}

export async function getMetrics(session: Session): Promise<Metric[]> {
	const response = await fetch(`${BACKEND_URL}/api/v1/metrics`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Metric[] = await response.json();
		return data;
	} else {
		return [];
	}
}

export async function getMetric(session: Session, metric_id: string): Promise<Metric | undefined> {
	const response = await fetch(`${BACKEND_URL}/api/v1/metrics/${metric_id}`, {
		headers: { Authorization: `Bearer ${session.auth_token}` }
	});
	if (response.ok) {
		const data: Metric = await response.json();
		return data;
	} else {
		return undefined;
	}
}

export async function get_coverage_for_metric(metric_id: String, session: Session) {
	try {
		const coverage_resp = await fetch(
			BACKEND_URL + `/api/v1/metrics/coverage_for_metric?metric_id=${metric_id}`,
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
