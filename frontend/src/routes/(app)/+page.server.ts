import { getSessionFromCookiesOrCreate } from '$lib/session_cookies';
import {
	get_control_completion,
	get_control_completion_batch,
	get_controls
} from '$lib/remote/controls';

export async function load({ cookies }) {
	const session = await getSessionFromCookiesOrCreate(cookies);

	const controls = await get_controls(session);
	const completion = new Map<string, number>();
	// for (const control of controls) {
	// 	completion.set(control.identifier, await get_control_completion(session, control.identifier));
	// }

	const completion_stats = await get_control_completion_batch(
		session,
		controls.map((c) => c.identifier)
	);

	for (let i = 0; i < controls.length; i++) {
		const control = controls[i];
		const comp = completion_stats[i];
		completion.set(control.identifier, comp);
	}

	return {
		controls,
		completion
	};
}
