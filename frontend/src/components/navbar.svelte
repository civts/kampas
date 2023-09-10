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
	<a class="rowcenter" href="/">
		<img src="/favicon.svg" alt="Kampas logo" />
		<p class="kampas">Kampas</p>
	</a>

	<div>
		<a class="addcontrol" href="/new_control">Add control</a>
		<a class="addcontrol" href="/new_metric">Add metric</a>
		<a class="addcontrol" href="/new_ranking">Create ranking</a>
	</div>

	<div class="rowcenter">
		{#if data?.user}
			{data.user}
			<img src="data:image/svg+xml;base64,{identicon_data}" alt="{data.user}'s profile picture" />
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
		p {
			margin: 0;
		}
	}

	img {
		border-radius: 50%;
		width: 2.5rem;
		height: 2.5rem;
	}

	a {
		color: var(--text-color);
		text-decoration: unset;
	}

	div {
		display: flex;
		align-content: center;
		gap: 1rem;
		a {
			padding: 0.7rem 1.3rem;
			align-self: center;
			text-decoration: unset;
		}
	}

	.rowcenter {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		p {
			font-size: 1.5rem;
		}
	}

	.addcontrol {
		background-color: var(--input-bg);
		border: none;
		color: var(--text-color);
		border-radius: 0.5rem;
		padding: 0.75rem 1rem;
		position: relative;
	}
	.kampas {
		font-weight: 600;
		letter-spacing: 0.01em;
	}
</style>
