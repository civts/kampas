// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			session: Session;
		}
		// interface PageData {}
		// interface Platform {}
	}

	type Session = {
		auth_token?: string;
		username?: string;
		id: string;
	};
}

export { Session };
