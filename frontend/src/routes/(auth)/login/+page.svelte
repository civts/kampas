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
	<form method="POST" use:enhance>
		{#if form?.username_error}
			<p class="error">{form.username_error}</p>
		{/if}
		<input type="text" name="username" placeholder="Username" required />
		{#if form?.password_error}
			<p class="error">{form.password_error}</p>
		{/if}
		<input type="password" name="password" placeholder="Password" required />

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
