<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<head>
	<title>Login</title>
</head>
<div>
	<h1>Login</h1>
	{#if form?.reason}
		<p class="error">{form.reason}</p>
	{/if}
	<form method="POST" use:enhance>
		{#if form?.username_error}
			<p class="error">{form.username_error}</p>
		{/if}
		<label for="username">Username</label>
		<input type="text" name="username" id="username" placeholder="Username" required />
		{#if form?.password_error}
			<p class="error">{form.password_error}</p>
		{/if}
		<label for="password">Password</label>
		<input type="password" name="password" id="password" placeholder="Password" required />

		<button type="submit">Login</button>
	</form>
	<p>Or register <a href="/register">here</a></p>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1em;
	}
</style>
