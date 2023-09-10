import { BACKEND_URL } from '$env/static/private';
import type { Ranking } from '$lib/models/bindings/Ranking';

export async function get_ranking(session: Session, id: String) {
	try {
		const rankings_resp = await fetch(BACKEND_URL + '/api/v1/rankings/' + id, {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let ranking: Ranking = await rankings_resp.json();
		return ranking;
	} catch (error) {
		console.error('Error loading ranking: ', error);
		return undefined;
	}
}

export async function get_rankings(session: Session) {
	try {
		const rankings_resp = await fetch(BACKEND_URL + '/api/v1/rankings/', {
			headers: {
				Authorization: `Bearer ${session.auth_token}`
			}
		});

		let rankings: Ranking[] = await rankings_resp.json();
		return rankings;
	} catch (error) {
		console.error('Error loading the rankings: ', error);
		return undefined;
	}
}
