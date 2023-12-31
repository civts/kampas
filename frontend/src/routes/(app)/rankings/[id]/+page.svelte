<script lang="ts">
	import type { Control } from '$lib/models/bindings/Control';
	import type { Measure } from '$lib/models/bindings/Measure';
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import AddTagButton from '../../../../components/add_tag_button.svelte';
	import Tag from '../../../../components/tag.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let filter_tags: TagI[] = [];
	let any_tag: boolean = false;

	async function add_tag(tag: TagI) {
		if (filter_tags.findIndex((t) => t.identifier == tag.identifier) == -1) {
			filter_tags.push(tag);
			filter_tags = filter_tags;
		}

		return true;
	}

	const filter_by_tag = (
		m: Control | Measure | undefined,
		tags: Map<String, TagI[]>,
		filter_tags: TagI[],
		any_tag: boolean
	) => {
		if (filter_tags.length == 0) {
			return true;
		}
		if (m == undefined) {
			return false;
		}
		const tags_for_this = tags.get(m.identifier);
		const sameTag = (tag: TagI): boolean => {
			return tags_for_this?.find((t) => t.identifier == tag.identifier) != undefined;
		};
		if (any_tag) {
			// Show the measures that have at least one of the tags
			const matching_tag = filter_tags.find(sameTag);
			return matching_tag != undefined;
		} else {
			// Show only the measures that have all the tags
			const unmatching_tag = filter_tags.find((tag) => !sameTag(tag));
			return unmatching_tag == undefined;
		}
	};

	$: filtered_measures =
		data.measures
			.filter((t) => filter_by_tag(t, data.measures_tags, filter_tags, any_tag))
			.sort((a, b) => a.title.localeCompare(b.title)) ?? [];

	$: filtered_controls = data.controls
		.filter((c) => filter_by_tag(c, data.control_tags, filter_tags, any_tag))
		.sort((a, b) => a.title.localeCompare(b.title));

	$: average_progress =
		filtered_measures.reduce((prev, m) => {
			return prev + (m?.progress ?? 0);
		}, 0) / filtered_measures.length;

	$: rounded_average_progress = (Math.round(average_progress * 100) / 100).toFixed(2);
</script>

<head>
	<title>Ranking</title>
</head>

<section>
	{#if data.ranking}
		<h1>{data.ranking.name}</h1>
		<span>Created by: {data.ranking.created_by}</span>
		<br />
		<span>Created on: {new Date(Number.parseInt(data.ranking.created_at)).toUTCString()}</span>
		<br />
		<span>Ordering: {data.ranking.ordering}</span>
		<br />
		<span>Minimum coverage: {data.ranking.minimum_coverage}</span>
		<section>
			<h2 class="lessmargin">Filter by tag</h2>
			<p>Click on a tag to remove it</p>
			<div class="row">
				{#each filter_tags as tag}
					<Tag
						{tag}
						close_callback={async (tag) => {
							const index = filter_tags.indexOf(tag);
							if (index != -1) {
								filter_tags.splice(index, 1);
								filter_tags = filter_tags;
							}
						}}
					/>
				{/each}
				<AddTagButton callback={add_tag} />
				<input type="checkbox" class="toggle" bind:checked={any_tag} />
				{#if any_tag}
					Any tag
				{:else}
					All the tags
				{/if}
			</div>
		</section>
		<section>
			<h2>Measures</h2>
			{#if filtered_measures.length > 0}
				<p>Average progress of these measures: {rounded_average_progress}%</p>
				<ol>
					{#each filtered_measures as m}
						<li>
							<a href="/measures/{m?.identifier}">{m?.title}</a> (progress: {m?.progress}%)
						</li>
					{/each}
				</ol>{:else}
				Nothing to show, try filtering for something else
			{/if}
		</section>
		<section>
			<h2>Controls</h2>
			{#if filtered_controls.length > 0}
				<p>
					If all the previous measures are completed, the following controls will be completed as
					well. <br /> Most likely, other controls will be improved too when implementing the measures.
				</p>

				<ol>
					{#each filtered_controls as c}
						<li>
							<a href="/controls/{c?.identifier}">{c?.title}</a>
						</li>
					{/each}
				</ol>
			{:else}
				Nothing to show, try filtering for something else
			{/if}
		</section>
	{:else}
		No ranking for that name. Maybe it does not exist, maybe you don't have access.
	{/if}
</section>

<style lang="scss">
	.lessmargin {
		margin-bottom: 0.5rem;
	}
	.row {
		justify-content: flex-start;
		gap: 1rem;
	}
	li {
		margin-bottom: 0.7rem;
	}
</style>
