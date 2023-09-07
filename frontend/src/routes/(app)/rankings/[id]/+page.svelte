<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Ranking</title>
</head>

<section>
	{#if data.ranking}
		<h2>Metrics</h2>
		<ol>
			{#each data.metrics || [] as m}
				<li>
					<a href="/metrics/{m?.identifier}">{m?.title}</a>
				</li>
			{/each}
		</ol>
		<span>Created by: {data.ranking.created_by}</span>
		<br />
		<span>Ordering: {data.ranking.ordering}</span>
	{:else}
		No ranking for that name. Maybe it does not exist, maybe you don't have access.
	{/if}
</section>
