<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Rankings</title>
</head>

<h1>Rankings</h1>
<section>
	{#each data?.rankings || [] as ranking}
		<div>
			<a href="/rankings/{ranking.identifier}"><h1>{ranking.name}</h1></a>
			<span
				>{new Date(Number.parseInt(ranking.created_at)).toUTCString()}, {ranking.created_by}</span
			>
		</div>
	{/each}
</section>

<style>
	div {
		margin: 1rem;
	}
</style>
