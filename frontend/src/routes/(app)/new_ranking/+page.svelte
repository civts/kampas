<script lang="ts">
	import { enhance } from '$app/forms';
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import { afterUpdate } from 'svelte';
	import AddTagButton from '../../../components/add_tag_button.svelte';
	import Tag from '../../../components/tag.svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;
	let alltags: boolean = false;
	let min_coverage = 100;
	let filter_by_tag: boolean = true;

	let selected_tags: TagI[] = [];

	async function addTag(tag: TagI): Promise<boolean> {
		if (selected_tags.indexOf(tag) == -1) {
			selected_tags.push(tag);
			// So that svelte updates the UI. See https://stackoverflow.com/a/69791545
			selected_tags = selected_tags;
		}

		return true;
	}

	afterUpdate(() => {
		console.log('Running againn');
		if (form?.ranking_id) {
			const to = `/rankings/${form.ranking_id}`;
			console.log('Redirecting', to);
			window.location.href = to;
		}
	});
</script>

<head>
	<title>Create Ranking</title>
</head>

<section>
	<h1>Add a new ranking</h1>
	<div class="createform">
		<form action="?/add" method="post" use:enhance>
			<label for="name">Ranking name</label>
			<input type="text" name="name" placeholder="name" />
			<label for="minimum_coverage">Minimum coverage (for each control)</label>
			<input
				type="number"
				min="1"
				max="100"
				name="minimum_coverage"
				bind:value={min_coverage}
				placeholder="Mimimum coverage for each control"
			/>
			<div class="row">
				<input type="checkbox" name="filter_by_tag" class="toggle" bind:checked={filter_by_tag} />
				<span>
					{#if filter_by_tag}
						Filtering by tag
					{:else}
						Not filtering by tag
					{/if}
				</span>
			</div>
			{#if filter_by_tag}
				<div class="tagrow">
					{#each selected_tags as tag}
						<Tag {tag} />
						<input type="text" name="tag" hidden value={tag.identifier} />
					{/each}
					<AddTagButton callback={addTag} />
				</div>
				<div class="row">
					<input type="checkbox" name="all_tags" class="toggle" bind:checked={alltags} />
					<span>
						{#if alltags}
							All the tags above
						{:else}
							Any of the tags above
						{/if}
					</span>
				</div>
			{/if}
			<button type="submit">Okay</button>
		</form>
		{#if form?.reason}
			<p class="error">Failed: {form?.reason}</p>
		{/if}
	</div>
</section>

<style lang="scss">
	input.toggle {
		$h: 2em;
		width: 2 * $h;
		height: $h;
		margin: 0;
		border: unset;
		position: relative;
		&::before {
			content: '';
			background-color: var(--input-bg);
			border-radius: 2 * $h;
			position: absolute;
			width: 100%;
			height: 100%;
		}
		&::after {
			position: absolute;
			content: '';
			box-sizing: border-box;
			background-color: var(--input-bg);
			border: 3px solid var(--accent);
			width: $h;
			height: $h;
			border-radius: $h;
			transition: all 0.2s ease;
			transform: translateX(0);
			left: 0;
		}
		&:checked::after {
			background-color: var(--accent);
			transform: translateX($h);
		}
	}
	.row {
		gap: 1rem;
		span {
			width: 20ch;
		}
	}
	.tagrow {
		display: flex;
		flex-wrap: wrap;
		align-content: flex-start;
		gap: 1rem;
		background-color: var(--input-bg);
		min-width: 19rem;
		max-width: 35rem;
		min-height: 16rem;
		border-radius: 0.5rem;
		padding: 1rem;
	}
</style>
