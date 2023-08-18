<script lang="ts">
	import { BACKEND_URL } from '$lib/costants';
</script>

<div>
	<h1>Login</h1>

	<form action="{BACKEND_URL}/api/v1/auth/login" method="post">
		<input type="text" name="username" id="newControl" placeholder="Username" />
		<input type="password" name="password" placeholder="Password" />
		<button type="submit">Login</button>
	</form>

	<p>Or register <a href="/register">here</a></p>
</div>

<style>
	div {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 2em;
	}
</style>
