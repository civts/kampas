import { createNewSession, getSession, updateSession } from '$lib/session_redis';
import type { Cookies } from '@sveltejs/kit';

const SESSION_COOKIE_NAME = 'session_id';

export async function getSessionFromCookiesOrCreate(cookies: Cookies): Promise<Session> {
	let session = await getSessionFromCookies(cookies);
	if (session == null) {
		session = await createNewSession();
		await updateSession(session);
		setSessionCookie(cookies, session.id);
	}
	return session;
}

async function getSessionFromCookies(cookies: Cookies): Promise<Session | null> {
	const sessionId = await extractSessionId(cookies);

	if (sessionId) {
		return getSession(sessionId);
	}

	console.log('No session id in the cookies');
	return null;
}

async function extractSessionId(cookies: Cookies): Promise<string | undefined> {
	const sessionId = cookies.get(SESSION_COOKIE_NAME);
	return sessionId;
}

export function setSessionCookie(cookies: Cookies, session_id: string) {
	cookies.set(SESSION_COOKIE_NAME, session_id, {
		httpOnly: true,
		sameSite: 'strict',
		secure: process.env.NODE_ENV == 'production',
		maxAge: 60 * 60 * 24 * 7
	});
}
