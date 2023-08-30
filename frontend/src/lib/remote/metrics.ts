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
		console.log('METRICS METRICS METRICS METRICS METRICS ', data);
		return data;
	} else {
		return [];
	}
}
