<script lang="ts">
	import Identicon from 'identicon.js';

	type userdata = {
		user: string | undefined;
		identicon_data: string;
	};
	export let data: userdata | undefined;

	const identicon_data = new Identicon(data?.identicon_data || '', {
		size: 100,
		format: 'svg'
	}).toString();
</script>

<nav>
	<a href="/">
		<img src="/favicon.svg" alt="Kartik logo" height="50px" />
		Kartik
	</a>

	<div>
		{#if data?.user}
			<p>{data.user}</p>
			<img
				width="50"
				height="50"
				src="data:image/svg+xml;base64,{identicon_data}"
				alt="{data.user}'s profile picture"
			/>
		{:else}
			<a href="/login">Login</a>
		{/if}
	</div>
</nav>

<style lang="scss">
	nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}
	img {
		border-radius: 50%;
	}
	div {
		display: flex;
		align-content: center;
		gap: 1em;
		a {
			background-color: var(--bg2);
			padding: 0.7em 1.3em;
			border-radius: 0.5em;
			align-self: center;
			text-decoration: unset;
		}
	}
</style>
