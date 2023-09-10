<script lang="ts">
	import Tag from '../../../../components/tag.svelte';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;

	$: measure = data.measure;

	let edit_form_shown = false;
</script>

<head>
	<title>{measure?.title || 'Measure details'}</title>
</head>

{#if measure}
	<section>
		<h1>{measure.title}</h1>
		<p>{measure.description}</p>
		<ul>
			<li>Progress: {measure.progress}</li>
			<li>Effort: {measure.effort}</li>
		</ul>
		<button
			class="btn"
			type="button"
			on:click={(_) => {
				edit_form_shown = !edit_form_shown;
			}}
		>
			Edit
		</button>
		{#if edit_form_shown}
			<form action="?/edit_measure" method="post">
				<label for="title">Title</label>
				<input type="text" name="title" id="title" placeholder="title" value={measure.title} />
				<label for="description">Description</label>
				<input
					type="text"
					name="description"
					id="description"
					placeholder="description"
					value={measure.description}
				/>
				<label for="effort">Effort</label>
				<input
					type="number"
					min="1"
					name="effort"
					id="effort"
					placeholder="effort"
					value={measure.effort}
				/>
				<label for="progress">Progress</label>
				<input
					type="number"
					min="0"
					max="100"
					name="progress"
					id="progress"
					placeholder="progress"
					value={measure.progress}
				/>
				<input hidden type="text" name="id" bind:value={measure.identifier} />

				<button type="submit">Update</button>
			</form>
			{#if form?.success}
				Measure updated successfully
			{/if}
			{#if form?.reason}
				<p class="error">{form.reason}</p>
			{/if}
		{/if}
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
	Loading measure data
{/if}

<style lang="scss">
	div.row {
		justify-content: start;
		gap: 1rem;
		margin-top: 1rem;
	}

	button.btn {
		background-color: var(--input-bg);
		border: none;
		color: var(--text-color);
		border-radius: 0.5rem;
		padding: 0.75rem 1rem;
		margin-top: 0;
	}
</style>
