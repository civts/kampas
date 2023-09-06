<script lang="ts">
	import { enhance } from '$app/forms';
	import TagControlButton from '../../../../components/tag_control_button.svelte';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: control = data.control;
</script>

<head>
	<title>{control?.title || 'Control details'}</title>
</head>

{#if control}
	<h1>{control.title}</h1>
	<p>{control.description}</p>
	<TagControlButton control_id={control.identifier} />
	<section>
		<h1>Associated Metrics</h1>
		<ul>
			{#each data.metrics as metric}
				<li>
					<span>{metric.title}</span>
					<span>{metric.description}</span>
				</li>
			{/each}
		</ul>
	</section>
	<section>
		<h1>Add metric</h1>
		<form action="?/add" method="post" use:enhance>
			{#if form?.title_error}
				<p class="error">{form.title_error}</p>
			{/if}
			<label for="title">Title</label>
			<input type="text" name="title" placeholder="Title" />
			{#if form?.description_error}
				<p class="error">{form.description_error}</p>
			{/if}
			<label for="Description">Description</label>
			<input type="text" name="description" placeholder="Description" />
			{#if form?.effort_error}
				<p class="error">{form.effort_error}</p>
			{/if}
			<label for="effort">Effort</label>
			<input type="number" min="1" name="effort" placeholder="Effort" />
			<label for="coverage">Coverage</label>
			<input
				type="number"
				min="1"
				max="100"
				value="100"
				name="coverage"
				placeholder="Coverage (1-100)"
			/>
			{#if form?.coverage_error}
				<p class="error">{form.coverage_error}</p>
			{/if}
			<input type="text" hidden name="control_id" bind:value={control.identifier} />
			<button type="submit">Add</button>
			{#if form?.success}
				<p>Metric added successfully</p>
			{/if}
			{#if form?.reason}
				<p class="error">Adding the metric failed: {form.reason}</p>
			{/if}
		</form>
	</section>
{/if}

<style lang="scss">
	ul {
		list-style: none;
		display: flex;
		gap: 1rem;
		padding-left: 0;
		li {
			border-radius: 0.5rem;
			background: var(--bg2);
			padding: 1rem 2rem;
		}
	}
	form {
		display: flex;
		width: 300px;
		margin: 0rem auto;
		align-items: start;
	}
</style>
