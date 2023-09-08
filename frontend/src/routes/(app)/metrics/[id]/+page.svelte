<script lang="ts">
	import { enhance } from '$app/forms';
	import Tag from '../../../../components/tag.svelte';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	// export let form: ActionData;

	$: metric = data.metric;
</script>

<head>
	<title>{metric?.title || 'Metric details'}</title>
</head>

{#if metric}
	<section>
		<h1>{metric.title}</h1>
		<p>{metric.description}</p>
		<ul>
			<li>Progress: {metric.progress}</li>
			<li>Effort: {metric.effort}</li>
		</ul>
	</section>
	<section>
		<h1>Associated Controls</h1>
		<div class="row">
			{#each data.controls as control}
				<a href="/controls/{control.identifier}">
					{control.title} (coverage: {data.coverage.get(control.identifier)}%)
				</a>
			{/each}
		</div>
	</section>
	<section>
		<h1>Tags</h1>
		(These are the tags of all the associated controls)
		<div class="row">
			{#each data.tags as tag}
				<Tag {tag} />
			{/each}
		</div>
	</section>
{:else}
	Loading metric data
{/if}

<style lang="scss">
	div.row {
		justify-content: start;
		gap: 1rem;
		margin-top: 1rem;
	}
</style>
