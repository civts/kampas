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
