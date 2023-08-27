<script lang="ts">
	import BarChart from '../../components/bar_chart.svelte';
	import RoundGauge from '../../components/round_gauge.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const controls_count = data.controls.length;
	const sum = data.controls.reduce((total, control) => total + control.progress, 0);

	let average_completion = 0;
	if (controls_count != 0) {
		average_completion = sum / data.controls.length;
	}
	let progress = data.controls.map((control) => control.progress);

	const controls_completed = data.controls.filter((c) => c.progress == 100).length;
</script>

<head>
	<title>Kartik frontend</title>
</head>

{#if data?.user}
	<p>The company has {controls_count} controls</p>
	<div class="row">
		<div>
			<RoundGauge
				value={controls_completed}
				max_value={controls_count}
				color="green"
				text={`${controls_completed} / ${controls_count}`}
			/>
			<p>{controls_completed} of these have been completed</p>
		</div>

		<div>
			<RoundGauge value={average_completion} color="purple" />
			<p>On average, they are {Math.round(average_completion)}% complete</p>
		</div>

		{#if controls_count != 0}
			<div>
				<div class="barchart">
					<BarChart data={progress} />
				</div>
				<p>Here is the distribution of how complete they are</p>
			</div>
		{/if}
	</div>

	<p>See the complete list of <a href="/controls">all controls</a></p>
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
