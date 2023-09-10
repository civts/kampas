import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import {
	get_control_completion_batch,
	get_controls,
	get_number_of_metrics_per_control_batch
} from '$lib/remote/controls';
import { getMetrics, get_control_associated_to_metric_batch } from '$lib/remote/metrics';
import { get_tags_for_control_batch, get_tags_for_metric_batch } from '$lib/remote/tags';

export async function load({ cookies }) {
	const session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);

	const completion = new Map<string, number>();
	const control_ids = controls.map((c) => c.identifier);
	const completion_stats = await get_control_completion_batch(session, control_ids);
	for (let i = 0; i < controls.length; i++) {
		const control = controls[i];
		const comp = completion_stats[i];
		completion.set(control.identifier, comp);
	}

	const metrics = await getMetrics(session);
	metrics.sort((a, b) => {
		if (a.title.toLocaleLowerCase() > b.title.toLocaleLowerCase()) {
			return 1;
		}
		return -1;
	});
	const metric_ids = metrics.map((m) => m.identifier);
	const number_of_controls_per_metric = new Map<String, number>();
	const associated_number = await get_control_associated_to_metric_batch(session, metric_ids);
	for (let i = 0; i < metric_ids.length; i++) {
		const mid = metric_ids[i];
		const associated_controls = associated_number[i];
		number_of_controls_per_metric.set(mid, associated_controls);
	}

	const number_of_metrics_per_control = await get_number_of_metrics_per_control_batch(session);

	const tags_for_control = await get_tags_for_control_batch(control_ids, session);
	const tags_for_metric = await get_tags_for_metric_batch(session);

	return {
		controls,
		completion,
		metrics,
		number_of_controls_per_metric,
		number_of_metrics_per_control,
		tags_for_control,
		tags_for_metric
	};
}
