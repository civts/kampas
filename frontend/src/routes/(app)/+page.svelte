<script lang="ts">
	import type { Control } from '$lib/models/bindings/Control';
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import AddTagButton from '../../components/add_tag_button.svelte';
	import BarChart from '../../components/bar_chart.svelte';
	import RoundGauge from '../../components/round_gauge.svelte';
	import Tag from '../../components/tag.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let selected_tags: TagI[] = [];
	let filtered_controls = data.controls;

	let controls_progress: number[] = [];

	let controls_count = 0;
	let average_completion = 0;
	let median_completion = 0;
	let controls_completed = 0;

	let measures_count = 0;
	let measures_completed = 0;
	let measures_avg_completion = 0;
	let measures_median_completion = 0;

	let sortedColumn = 'title';
	let sortDirection = 1;
	let sorted_controls: Control[] = [];

	let sortedControlsColumn = 'progress';
	let sortControlsDirection = 1;
	let sortedMeasures = data.measures;
	updateMeasures();
	updateControls();

	async function addTag(tag: TagI) {
		if (selected_tags.findIndex((t) => t.identifier == tag.identifier) == -1) {
			selected_tags.push(tag);
			selected_tags = selected_tags;
			selected_tags.sort((a, b) => a.name.localeCompare(b.name));
			updateMeasures();
			updateControls();
		}
		return true;
	}

	async function removeTag(tag: TagI) {
		const index = selected_tags.indexOf(tag);
		if (index != -1) {
			selected_tags.splice(index, 1);
			selected_tags = selected_tags;
			updateMeasures();
			updateControls();
		}
	}

	function updateControls() {
		filtered_controls = data.controls.filter((control) => {
			let s = new Set(
				data.tags_for_control.get(control.identifier)?.map((t) => t.identifier) ?? []
			);
			return selected_tags.every((t) => s.has(t.identifier));
		});

		updateControlsStats();

		sortControlsTable('title');
	}

	function updateControlsStats() {
		controls_count = filtered_controls.length;
		const sum = filtered_controls.reduce(
			(total, control) => total + (data.completion.get(control.identifier) || 0),
			0
		);
		if (controls_count != 0) {
			average_completion = sum / controls_count;
			let prog = filtered_controls
				.map((control) => data.completion.get(control.identifier) || 0)
				.sort();
			median_completion = prog[Math.floor(prog.length / 2)];
		} else {
			average_completion = 0;
			median_completion = 0;
		}

		controls_completed = filtered_controls.filter(
			(c) => (data.completion.get(c.identifier) || 0) >= 99.99999999
		).length;
		controls_progress = filtered_controls.map(
			(control) => data.completion.get(control.identifier) || 0
		);
		sorted_controls = filtered_controls;
	}

	function sortControlsTable(column: string) {
		if (sortedControlsColumn === column) {
			sortControlsDirection *= -1;
		} else {
			sortControlsDirection = 1;
			sortedControlsColumn = column;
		}
		sorted_controls = sorted_controls.sort((a, b) => {
			if (column == 'title') {
				return a.title.localeCompare(b.title) * sortControlsDirection;
			} else if (column === 'measures') {
				const na = data.number_of_measures_per_control.get(a.identifier) ?? 0;
				const nb = data.number_of_measures_per_control.get(b.identifier) ?? 0;
				const cmp = na - nb;
				return cmp * sortControlsDirection;
			} else if (column === 'controlprogress') {
				const na = data.completion.get(a.identifier) ?? 0;
				const nb = data.completion.get(b.identifier) ?? 0;
				const cmp = na - nb;
				return cmp * sortControlsDirection;
			}
			return 0;
		});
	}

	function updateMeasures() {
		sortedMeasures = data.measures.filter((measure) => {
			let s = new Set(data.tags_for_measure.get(measure.identifier) ?? []);
			return selected_tags.every((t) => s.has(t.identifier));
		});
		updateMeasuresStats();
	}

	function updateMeasuresStats() {
		measures_count = sortedMeasures.length;
		const measures_sum = sortedMeasures.reduce((total, measure) => {
			return total.valueOf() + measure.progress;
		}, 0);
		if (measures_count != 0) {
			measures_avg_completion = measures_sum.valueOf() / measures_count;
			let measures_by_progress = sortedMeasures.sort((m1, m2) => m1.progress - m2.progress);
			measures_median_completion =
				measures_by_progress[Math.floor(measures_by_progress.length / 2)].progress;
		} else {
			measures_avg_completion = 0;
			measures_median_completion = 0;
		}

		measures_completed = sortedMeasures.filter((m) => {
			return m.progress >= 99.99999999;
		}).length;
	}

	function sortMeasuresTable(column: string) {
		if (sortedColumn === column) {
			sortDirection *= -1;
		} else {
			sortDirection = 1;
			sortedColumn = column;
		}

		if (column === 'title') {
			sortedMeasures.sort((a, b) => a.title.localeCompare(b.title) * sortDirection);
		} else if (column === 'progress') {
			sortedMeasures.sort((a, b) => (a.progress - b.progress) * sortDirection);
		} else if (column === 'controls') {
			sortedMeasures.sort(
				(a, b) =>
					((data.number_of_controls_per_measure.get(a.identifier) || 0) -
						(data.number_of_controls_per_measure.get(b.identifier) || 0)) *
					sortDirection
			);
		}
		sortedMeasures = sortedMeasures;
	}
</script>

<head>
	<title>Kampas dashboard</title>
</head>

{#if data?.user}
	<div class="row start">
		{#each selected_tags as tag}
			<Tag {tag} close_callback={removeTag} />
		{/each}
		<AddTagButton callback={addTag} />
	</div>
	<section>
		<h1>Controls</h1>
		<p>
			The company has {controls_count} controls
			{#if selected_tags.length > 0}
				with the selected tags
			{/if}
		</p>

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
				<RoundGauge value={average_completion} percent={true} color="purple" />
				<p>On average, they are {Math.round(average_completion)}% complete</p>
			</div>

			<div>
				<RoundGauge value={median_completion} percent={true} color="#b36200" />
				<p>The median completion is {Math.round(median_completion)}%</p>
			</div>

			{#if controls_count > 1}
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
		<h1>Measures</h1>
		<p>
			The company has {measures_count} measures
			{#if selected_tags.length > 0}
				with the selected tags
			{/if}
		</p>

		<div class="row">
			<div>
				<RoundGauge
					value={measures_completed}
					max_value={measures_count}
					color="#9ff"
					text={`${measures_completed} / ${measures_count}`}
				/>
				<p>{measures_completed} of these have been completed</p>
			</div>

			<div>
				<RoundGauge value={measures_avg_completion} percent={true} color="#99f" />
				<p>On average, they are {Math.round(measures_avg_completion)}% complete</p>
			</div>

			<div>
				<RoundGauge value={measures_median_completion} percent={true} color="#f9f" />
				<p>The median completion is {measures_median_completion}%</p>
			</div>

			{#if measures_count > 1}
				<div>
					<div class="barchart">
						<BarChart data={sortedMeasures.map((m) => m.progress)} />
					</div>
					<p>Here is the distribution of how complete they are</p>
				</div>
			{/if}
		</div>

		{#if sortedMeasures.length > 0}
			<table cellspacing="0">
				<tr>
					<th on:click={() => sortMeasuresTable('title')}>
						Measure title
						{#if sortedColumn === 'title'}
							<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
					<th on:click={() => sortMeasuresTable('progress')}>
						Progress
						{#if sortedColumn === 'progress'}
							<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
					<th on:click={() => sortMeasuresTable('controls')}>
						# of associated controls
						{#if sortedColumn === 'controls'}
							<span class={sortDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
				</tr>
				{#each sortedMeasures as measure}
					<tr>
						<td><a href="/measures/{measure.identifier}">{measure.title}</a></td>
						<td>{measure.progress}</td>
						<td>{data.number_of_controls_per_measure.get(measure.identifier)}</td>
					</tr>
				{/each}
			</table>
		{/if}
	</section>

	{#if sorted_controls.length > 0}
		<section>
			<h1>Controls and Measures</h1>
			<table cellspacing="0">
				<tr>
					<th on:click={() => sortControlsTable('title')}>
						Control title
						{#if sortedControlsColumn === 'title'}
							<span class={sortControlsDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
					<th on:click={() => sortControlsTable('measures')}>
						# of associated measures
						{#if sortedControlsColumn === 'measures'}
							<span class={sortControlsDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
					<th on:click={() => sortControlsTable('controlprogress')}>
						Progress
						{#if sortedControlsColumn === 'controlprogress'}
							<span class={sortControlsDirection === 1 ? 'arrow down' : 'arrow up'} />
						{/if}
					</th>
				</tr>
				{#each sorted_controls as control}
					<tr>
						<td>
							<a href="/controls/{control.identifier}">{control.title}</a>
						</td>
						<td>
							{data.number_of_measures_per_control.get(control.identifier) ?? 0}
						</td>
						<td>
							{(Math.round((data.completion.get(control.identifier) || 0) * 100) / 100).toFixed(2)}%
						</td>
					</tr>
				{/each}
			</table>
		</section>

		<section>
			To see a graph view of the measure/controls relations, go <a
				data-sveltekit-preload-code="off"
				data-sveltekit-preload-data="off"
				href="/graph">here</a
			>
		</section>
	{/if}
{:else}
	<center>Sry, who are you?</center>
{/if}

<style lang="scss">
	.barchart {
		display: flex;
		justify-content: center;
		align-items: center;
	}
	section {
		margin-top: 3rem;
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
			max-width: 75ch;
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

	.start {
		justify-content: start;
		gap: 1rem;
	}
</style>
