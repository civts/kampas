import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { createHash } from 'crypto';
import { get_controls } from '$lib/remote/controls';

export async function load({ cookies }) {
	const session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);

	return {
		user: session.username,
		user_hash: get_username_hash(session.username || 'none'),
		controls
	};

	function get_username_hash(username: string) {
		const salt = '!35ÌÀ¯#%0y½Ò,¶Ê?l[È_ýì';
		const username_hash = createHash('sha256');
		username_hash.update(username);
		username_hash.update(salt);
		return username_hash.digest('hex');
	}
}
