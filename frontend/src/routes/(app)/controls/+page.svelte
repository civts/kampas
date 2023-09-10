<script lang="ts">
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import { afterUpdate } from 'svelte';
	import AddTagButton from '../../../components/add_tag_button.svelte';
	import ControlListItem from '../../../components/control_list_item.svelte';
	import Tag from '../../../components/tag.svelte';
	import type { PageData } from './$types';

	export let data: PageData;

	let filter_tags: TagI[] = [];
	let controls = data.controls;

	async function removeTag(tag: TagI) {
		const index = filter_tags.indexOf(tag);
		if (index != -1) {
			filter_tags.splice(index, 1);
			filter_tags = filter_tags;
		}
	}

	async function addTag(tag: TagI) {
		if (filter_tags.findIndex((t) => t.identifier == tag.identifier) == -1) {
			filter_tags.push(tag);
			filter_tags = filter_tags;
		}
		return true;
	}

	afterUpdate(() => {
		if (filter_tags.length > 0) {
			const ids = filter_tags.map((t) => t.identifier);
			controls = data.controls.filter((c) => {
				const tags = new Set((data.tags.get(c.identifier) ?? []).map((t) => t.identifier));
				for (const tag of ids) {
					if (!tags.has(tag)) {
						return false;
					}
				}
				return true;
			});
		} else {
			controls = data.controls;
		}
		controls = controls.sort((a, b) => a.title.localeCompare(b.title));
	});
</script>

<head>
	<title>Controls</title>
</head>

<section>
	<p>Filter by tag:</p>
	<div class="row">
		{#each filter_tags as tag}
			<Tag {tag} close_callback={removeTag} />
		{/each}
		<AddTagButton callback={addTag} />
	</div>
</section>
<section>
	<p>Controls to implement:</p>
	<ol>
		{#each controls || [] as control}
			<li>
				<ControlListItem {control} tags={data.tags.get(control.identifier)} />
			</li>
		{:else}
			<p>Nothing to show, try filtering with different tags</p>
		{/each}
	</ol>
</section>

<style>
	.row {
		gap: 1rem;
		justify-content: start;
	}
</style>
