<script lang="ts">
	import BarChart from '../../components/bar_chart.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const controls_count = data.controls.length;
	const sum = data.controls.reduce((total, control) => total + control.progress, 0);

	let average_completion = 0;
	if (controls_count != 0) {
		average_completion = sum / data.controls.length;
	}
	let progress = data.controls.map((control) => control.progress);
</script>

<head>
	<title>Kartik frontend</title>
</head>

{#if data?.user}
	<p>The company has {controls_count} controls</p>
	<p>on average, they are {average_completion}% complete</p>
	{#if controls_count != 0}
		<p>Here is the distribution of how complete they are</p>
		<div class="barchart">
			<BarChart data={progress} />
		</div>
	{/if}
	See the complete list of <a href="/controls">all controls</a>
	<!-- <p>And here is the graph of how complete they have been in time</p> -->
{:else}
	<center>Sry, who are you?</center>
{/if}

<style>
	.barchart {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
