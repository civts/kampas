<script lang="ts">
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import { onMount } from 'svelte';
	import AddMeasureButton from '../../../../components/add_measure_button.svelte';
	import AddTagButton from '../../../../components/add_tag_button.svelte';
	import Tag from '../../../../components/tag.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
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
				tags = tags.sort((a, b) => a.name.localeCompare(b.name));
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
	<p>Progress: {data.progress}%</p>
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
		<h1>Associated Measures</h1>
		<ul>
			{#each data.measures as measure}
				<li>
					<a href="/measures/{measure.identifier}">
						<span>{measure.title} (progress: {measure.progress}%)</span>
					</a>
				</li>
			{/each}
			<AddMeasureButton
				on_select={async (m, coverage) => {
					if (control != undefined) {
						const response = await fetch('/api/measures/associate', {
							method: 'POST',
							headers: { 'Content-Type': 'application/json' },
							body: JSON.stringify({
								measure_id: m.identifier,
								control_id: control.identifier,
								coverage: coverage
							})
						});

						if (response.ok) {
							if (data.measures.findIndex((metr) => metr.identifier == m.identifier) == -1) {
								data.measures.push(m);
								data.measures = data.measures;
							}
						} else {
							alert('Associating the measure to the control failed. Are they already associated?');
						}
						return response.ok;
					} else {
						return false;
					}
				}}
			/>
		</ul>
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

	.row {
		justify-content: start;
		gap: 1rem;
	}
</style>
