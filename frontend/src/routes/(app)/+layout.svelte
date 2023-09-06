<script lang="ts">
	import Navbar from '../../components/navbar.svelte';
	import '../../style/app.scss';
	import type { LayoutData } from './$types';

	export let data: LayoutData;
</script>

<header>
	<Navbar data={{ user: data.user, identicon_data: data.user_hash }} />
</header>
<main>
	<slot />
</main>

<style>
	main {
		margin-bottom: 10vh;
	}
	header {
		margin-bottom: 2rem;
	}
</style>
