<script lang="ts">
	import { enhance } from '$app/forms';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: metric = data.metric;
</script>

<head>
	<title>{metric?.title || 'Metric details'}</title>
</head>

{#if metric}
	<h1>{metric.title}</h1>
	<p>{metric.description}</p>
	<section>
		<h1>Associated Controls</h1>
		todo
	</section>
	<section>
		<ul>
			<li>Progress: {metric.progress}</li>
			<li>Effort: {metric.effort}</li>
		</ul>
	</section>
{:else}
	Loading metric data
{/if}
