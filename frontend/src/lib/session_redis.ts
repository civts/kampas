import { v4 as uuidv4 } from 'uuid';
import redis from 'redis';
import { REDIS_URL } from './costants';

const client = redis.createClient({
	url: REDIS_URL,
	socket: {
		reconnectStrategy: (retries) => Math.min(retries * 50, 1000)
	}
});
await client.connect();

export async function updateSession(session: Session) {
	await client.set(session.id, JSON.stringify(session));
}

export async function getSession(id: string): Promise<Session | null> {
	const res = await client.get(id);
	if (res == null) {
		return null;
	}
	try {
		return JSON.parse(res);
	} catch (err) {
		console.error('We have an invalid session in the db, ' + id + ': ' + err);
		return null;
	}
}

export async function createNewSession(): Promise<Session> {
	const sessionId = createSessionId();
	const sessionData: Session = { id: sessionId };
	await updateSession(sessionData);
	return sessionData;
}

function createSessionId(): string {
	return uuidv4();
}
