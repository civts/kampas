<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<section>
	{#if data.ranking}
		<h2>Metrics</h2>
		<ol>
			{#each data.ranking.metrics || [] as m}
				<li>
					{m}
				</li>
			{/each}
		</ol>
		<span>Ordering: {data.ranking.ordering}</span>
		<br />
		<span>Created by: {data.ranking.created_by}</span>
		<br />
		<span>Id: {data.ranking.identifier}</span>
	{:else}
		No ranking for that name. Maybe it does not exist, maybe you don't have access.
	{/if}
</section>
