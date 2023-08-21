import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import { createHash } from 'crypto';

export async function load({ cookies }) {
	let session = await getSessionFromCookiesOrCreate(cookies);

	const salt = '!35ÌÀ¯#%0y½Ò,¶Ê?l[È_ýì';
	const username_hash = createHash('sha256');
	username_hash.update(session.username || 'none');
	username_hash.update(salt);

	return {
		user: session.username,
		user_hash: username_hash.digest('hex')
	};
}
