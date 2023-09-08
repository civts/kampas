<script lang="ts">
	import { enhance } from '$app/forms';
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import { onMount } from 'svelte';
	import AddTagButton from '../../../../components/add_tag_button.svelte';
	import Tag from '../../../../components/tag.svelte';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;
	export let form: ActionData;
	let tags: TagI[] = [];

	$: control = data.control;

	async function addTag(tagData: TagI, control_id: String) {
		const response = await fetch('/api/tags/add_to_control', {
			method: 'POST',
			body: JSON.stringify({ tag_id: tagData.identifier, control_id }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok) {
			if (tags.findIndex((tag) => tagData.identifier == tag.identifier) == -1) {
				tags.push(tagData);
				tags = tags;
			}
		}

		return response.ok;
	}

	async function removeTag(tagData: TagI) {
		if (control != undefined) {
			const response = await fetch('/api/tags/add_to_control', {
				method: 'DELETE',
				body: JSON.stringify({ tag_id: tagData.identifier, control_id: control.identifier }),
				headers: {
					'Content-Type': 'application/json'
				}
			});

			if (response.ok) {
				const index = tags.indexOf(tagData);
				if (index != -1) {
					tags.splice(index, 1);
					tags = tags;
				}
			}
			return response.ok;
		}
	}

	onMount(async () => {
		if (control != undefined) {
			const response = await fetch(`/api/tags?control_id=${control.identifier}`);
			if (response.ok) {
				try {
					tags = await response.json();
				} catch (error) {
					console.log('Could not load the tags for ', control.identifier, ': ', error);
				}
			}
		}
	});
</script>

<head>
	<title>{control?.title || 'Control details'}</title>
</head>

{#if control}
	<h1>{control.title}</h1>
	<p>{control.description}</p>
	<div class="row">
		{#each tags as tag}
			<Tag {tag} close_callback={removeTag} />
		{/each}
		<AddTagButton
			callback={async (tag) => {
				if (control != undefined) {
					return addTag(tag, control.identifier);
				} else {
					return false;
				}
			}}
		/>
	</div>
	<section>
		<h1>Associated Metrics</h1>
		<ul>
			{#each data.metrics as metric}
				<li>
					<a href="/metrics/{metric.identifier}">
						<span>{metric.title} (progress: {metric.progress}%)</span>
					</a>
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
		gap: 1rem;
		li {
			border-radius: 0.5rem;
			padding: 1rem 2rem;
		}
	}
	form {
		display: flex;
		width: 300px;
		margin: 0rem auto;
		align-items: start;
	}
	.row {
		justify-content: start;
		gap: 1rem;
	}
</style>
