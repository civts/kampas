<script lang="ts">
	import type { Control } from '$lib/models/bindings/Control';
	import type { Tag as T } from '$lib/models/bindings/Tag';
	import { afterUpdate, onMount } from 'svelte';
	import Tag from './tag.svelte';
	import AddTagButton from './add_tag_button.svelte';

	export let control: Control;
	export let tags: T[] | undefined = undefined;

	async function addTag(tagData: T) {
		const response = await fetch('/api/tags/add_to_control', {
			method: 'POST',
			body: JSON.stringify({ tag_id: tagData.identifier, control_id: control.identifier }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok) {
			tags ??= [];
			if (tags.findIndex((tag) => tagData.identifier == tag.identifier) == -1) {
				tags.push(tagData);
				tags = tags;
			}
		}
		return response.ok;
	}

	async function removeTag(tagData: T) {
		const response = await fetch('/api/tags/add_to_control', {
			method: 'DELETE',
			body: JSON.stringify({ tag_id: tagData.identifier, control_id: control.identifier }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok) {
			tags ??= [];
			const index = tags.indexOf(tagData);
			if (index != -1) {
				tags.splice(index, 1);
				tags = tags;
			}
		}
		return response.ok;
	}

	afterUpdate(async () => {
		if (tags == undefined) {
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

<h3>{control.title}</h3>
<p>{control.description}</p>
<a href="/controls/{control.identifier}"> Details </a>
<ul>
	{#each tags || [] as tag}
		<li><Tag {tag} close_callback={removeTag} /></li>
	{/each}
	<AddTagButton callback={addTag} />
</ul>

<style lang="scss">
	ul {
		margin-top: 1rem;
		display: flex;
		list-style: none;
		gap: 1rem;
		align-items: center;
		padding-left: 0;
	}
</style>
