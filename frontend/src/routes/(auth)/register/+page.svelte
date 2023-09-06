<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;

	const username_pattern: string = '^[a-z0-9][a-z0-9_]{1,28}[a-z0-9]$';
	const password_pattern: string = '.{12,256}';

	async function handleSubmit(
		event: Event & { readonly submitter: HTMLElement | null } & {
			currentTarget: EventTarget & HTMLFormElement;
		}
	): Promise<any> {
		event.preventDefault(); // Prevent the form from submitting normally
	}
</script>

<head>
	<title>Register</title>
</head>
<div>
	<h1>Register</h1>

	{#if form?.reason}
		<p class="error">An internal error occourred: {form.reason}</p>
	{/if}

	<form method="POST" use:enhance>
		{#if form?.username_error}
			<p class="error">{form.username_error}</p>
		{/if}
		<label for="username">Username</label>
		<input type="text" name="username" placeholder="Username" required />
		{#if form?.password_error}
			<p class="error">{form.password_error}</p>
		{/if}
		<label for="password">Password</label>
		<input type="password" name="password" placeholder="Password" required />
		{#if form?.password_conf_error}
			<p class="error">{form.password_conf_error}</p>
		{/if}
		<label for="password_conf">Password (confirmation)</label>
		<input type="password" name="password_conf" placeholder="Confirm Password" required />
		<button type="submit">Create Account</button>
	</form>

	<p>Already registered? Login <a href="/login">here</a></p>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1em;
	}
</style>
