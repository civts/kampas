import { BACKEND_URL } from '$lib/costants';
import type { Control } from '$lib/models/bindings/Control';
import type { Metric } from '$lib/models/bindings/Metric';
import type { Ranking } from '$lib/models/bindings/Ranking';
import type { Tag } from '$lib/models/bindings/Tag';

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

export async function get_tags(session: Session) {
	try {
		const controls_resp = await fetch(BACKEND_URL + '/api/v1/tags', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let tags: Tag[] = await controls_resp.json();
		return tags;
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

export async function get_ranking(session: Session, id: String) {
	try {
		const rankings_resp = await fetch(BACKEND_URL + '/api/v1/rankings/' + id, {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let rankings: Ranking = await rankings_resp.json();
		return rankings;
	} catch (error) {
		console.error('Error loading ranking: ', error);
		return undefined;
	}
}

export async function get_controls_for_metric(metric_id: String, session: Session) {
	try {
		const controls_resp = await fetch(BACKEND_URL + `/api/v1/controls?metric_id=${metric_id}`, {
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

export async function get_tags_for_metric(metric_id: String, session: Session) {
	try {
		const tags_resp = await fetch(
			BACKEND_URL + `/api/v1/metrics/tags_for_metric?metric_id=${metric_id}`,
			{
				headers: {
					Authorization: `Bearer ${session.auth_token}`
				}
			}
		);
		const tag_ids: Tag[] = await tags_resp.json();
		return tag_ids;
	} catch (error) {
		console.error('Error loading tags for metric: ', error);
		return [];
	}
}
