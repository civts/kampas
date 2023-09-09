import { BACKEND_URL } from '$lib/costants';
import type { Tag } from '$lib/models/bindings/Tag';

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

export async function get_tags_for_control_batch(control_ids: string[], session: Session) {
	console.log('Requesting batch tags');
	const response = await fetch(`${BACKEND_URL}/api/v1/tags/get_tags_batch`, {
		method: 'POST',
		headers: { Authorization: `Bearer ${session.auth_token}`, 'Content-Type': 'application/json' },
		body: JSON.stringify(control_ids)
	});
	if (response.ok) {
		const result = new Map<string, Tag[]>();
		const data: Tag[][] = await response.json();

		for (let i = 0; i < data.length; i++) {
			const control_id = control_ids[i];
			const tags = data[i];
			result.set(control_id, tags);
		}

		return result;
	} else {
		return new Map();
	}
}
