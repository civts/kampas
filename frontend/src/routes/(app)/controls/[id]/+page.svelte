<script lang="ts">
	import type { Tag as TagI } from '$lib/models/bindings/Tag';
	import { onMount } from 'svelte';
	import AddMetricButton from '../../../../components/add_metric_button.svelte';
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
			<AddMetricButton
				on_select={async (m, coverage) => {
					if (control != undefined) {
						const response = await fetch('/api/metrics/associate', {
							method: 'POST',
							headers: { 'Content-Type': 'application/json' },
							body: JSON.stringify({
								metric_id: m.identifier,
								control_id: control.identifier,
								coverage: coverage
							})
						});

						if (response.ok) {
							if (data.metrics.findIndex((metr) => metr.identifier == m.identifier) == -1) {
								data.metrics.push(m);
								data.metrics = data.metrics;
							}
						} else {
							alert('Associating the metric to the control failed. Are they already associated?');
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
