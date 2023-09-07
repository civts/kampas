<script lang="ts">
	import type { Control } from '$lib/models/bindings/Control';
	import type { Tag as T } from '$lib/models/bindings/Tag';
	import { onMount } from 'svelte';
	import Tag from './tag.svelte';
	import AddTagButton from './add_tag_button.svelte';

	export let control: Control;
	let tags: T[] = [];

	async function addTag(tagData: T, control_id: String) {
		const response = await fetch('/api/tags/add_to_control', {
			method: 'POST',
			body: JSON.stringify({ tag_id: tagData.identifier, control_id }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		return response.ok;
	}

	onMount(async () => {
		const response = await fetch(`/api/tags?control_id=${control.identifier}`);
		if (response.ok) {
			try {
				tags = await response.json();
			} catch (error) {
				console.log('Could not load the tags for ', control.identifier, ': ', error);
			}
		}
	});
</script>

<h3>{control.title}</h3>
<p>{control.description}</p>
<a href="/controls/{control.identifier}"> Details </a>
<ul>
	{#each tags as tag}
		<li><Tag {tag} /></li>
	{/each}
	<AddTagButton callback={(tag) => addTag(tag, control.identifier)} />
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
