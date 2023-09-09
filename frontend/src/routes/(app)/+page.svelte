<script lang="ts">
	import { afterUpdate, onMount } from 'svelte';
	import BarChart from '../../components/bar_chart.svelte';
	import RoundGauge from '../../components/round_gauge.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	const controls_count = data.controls.length;
	const sum = data.controls.reduce(
		(total, control) => total + (data.completion.get(control.identifier) || 0),
		0
	);

	let average_completion = 0;
	if (controls_count != 0) {
		average_completion = sum / controls_count;
	}
	let controls_progress = data.controls.map(
		(control) => data.completion.get(control.identifier) || 0
	);

	const controls_completed = data.controls.filter(
		(c) => data.completion.get(c.identifier) || 0 >= 100
	).length;

	const metrics_count = data.metrics_progress.length;
	const metrics_sum = data.metrics_progress.reduce(
		(total, metric) => total.valueOf() + metric.valueOf(),
		0
	);
	let metrics_completion = 0;
	if (metrics_count != 0) {
		metrics_completion = metrics_sum.valueOf() / metrics_count;
	}

	const metrics_completed = data.metrics_progress.filter((c) => c >= 100).length;

	let sortedColumn = '';
	let sortDirection = 1;

	let sortedMetrics = [...data.metrics];

	function sortTable(column: string) {
		if (sortedColumn === column) {
			sortDirection *= -1;
		} else {
			sortDirection = 1;
			sortedColumn = column;
		}

		if (column === 'title') {
			sortedMetrics.sort((a, b) => a.title.localeCompare(b.title) * sortDirection);
		} else if (column === 'progress') {
			sortedMetrics.sort((a, b) => (a.progress - b.progress) * sortDirection);
		} else if (column === 'controls') {
			sortedMetrics.sort(
				(a, b) =>
					((data.number_of_controls_per_metric.get(a.identifier) || 0) -
						(data.number_of_controls_per_metric.get(b.identifier) || 0)) *
					sortDirection
			);
		}
		sortedMetrics = sortedMetrics;
	}

	onMount(() => {
		sortedMetrics = [...data.metrics];
	});
</script>

<head>
	<title>Kartik frontend</title>
</head>

{#if data?.user}
	<section>
		<h1>Controls</h1>
		<p>The company has {controls_count} controls</p>

		<p>See the complete list of <a href="/controls">all controls</a></p>

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
						<BarChart data={controls_progress} />
					</div>
					<p>Here is the distribution of how complete they are</p>
				</div>
			{/if}
		</div>
	</section>

	<section>
		<h1>Metrics</h1>
		<p>The company has {metrics_count} metrics</p>

		<div class="row">
			<div>
				<RoundGauge
					value={metrics_completed}
					max_value={metrics_count}
					color="#9ff"
					text={`${metrics_completed} / ${metrics_count}`}
				/>
				<p>{metrics_completed} of these have been completed</p>
			</div>

			<div>
				<RoundGauge value={metrics_completion} color="#99f" />
				<p>On average, they are {Math.round(metrics_completion)}% complete</p>
			</div>
		</div>

		<table cellspacing="0">
			<tr>
				<th on:click={() => sortTable('title')}>
					Metric
					{#if sortedColumn === 'title'}
						<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
					{/if}
				</th>
				<th on:click={() => sortTable('progress')}>
					Progress
					{#if sortedColumn === 'progress'}
						<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
					{/if}
				</th>
				<th on:click={() => sortTable('controls')}>
					# of associated controls
					{#if sortedColumn === 'controls'}
						<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
					{/if}
				</th>
			</tr>
			{#each sortedMetrics as metric}
				<tr>
					<td><a href="/metrics/{metric.identifier}">{metric.title}</a></td>
					<td>{metric.progress}</td>
					<td>{data.number_of_controls_per_metric.get(metric.identifier)}</td>
				</tr>
			{/each}
		</table>
	</section>
{:else}
	<center>Sry, who are you?</center>
{/if}

<style lang="scss">
	.barchart {
		display: flex;
		justify-content: center;
		align-items: center;
	}
	table {
		max-width: 100%;
		margin-top: 2rem;
		margin-inline: auto;
		align-self: center;
		justify-self: center;
		td,
		th {
			padding: 0.5rem 1rem;
		}
		tr {
			border-spacing: 0;
		}

		th {
			justify-content: start;
			cursor: pointer;
			.arrow {
				border-style: solid;
				border-width: 0.2em 0.2em 0 0;
				content: '';
				display: inline-block;
				height: 0.4em;
				left: 0.2em;
				position: relative;
				width: 0.4em;
			}

			.up {
				transform: rotate(135deg);
			}
			.down {
				transform: rotate(-45deg);
			}
		}

		tr:nth-child(even) {
			background-color: var(--input-bg);
		}
	}
</style>
