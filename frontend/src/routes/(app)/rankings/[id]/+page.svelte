<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Ranking</title>
</head>

<section>
	{#if data.ranking}
		<h1>{data.ranking.name}</h1>
		<span>Created by: {data.ranking.created_by}</span>
		<br />
		<span>Created on: {new Date(Number.parseInt(data.ranking.created_at)).toUTCString()}</span>
		<br />
		<span>Ordering: {data.ranking.ordering}</span>

		<h2>Metrics</h2>
		<ol>
			{#each data.metrics || [] as m}
				<li>
					<a href="/metrics/{m?.identifier}">{m?.title}</a>
				</li>
			{/each}
		</ol>
	{:else}
		No ranking for that name. Maybe it does not exist, maybe you don't have access.
	{/if}
</section>
