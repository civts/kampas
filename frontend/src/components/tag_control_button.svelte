<script lang="ts">
	import type { Tag as T } from '$lib/models/bindings/Tag';
	import { onMount } from 'svelte';
	import Tag from './tag.svelte';

	export let control_id: string;

	let showOverlay = false;
	let tags: T[] = [];

	async function fetchTags() {
		const response = await fetch('/api/tags');
		tags = await response.json();
	}

	async function addTag(tagData: T) {
		const response = await fetch('/api/tags/add_to_control', {
			method: 'POST',
			body: JSON.stringify({ tag_id: tagData.identifier, control_id }),
			headers: {
				'Content-Type': 'application/json'
			}
		});

		if (response.ok) {
			showOverlay = false;
		}
	}

	onMount(fetchTags);
</script>

<button on:click={() => (showOverlay = true)}>Add tag</button>

{#if showOverlay}
	<div class="overlay" on:click={() => (showOverlay = false)}>
		<div class="overlay-content" on:click={(e) => e.stopPropagation()}>
			<div class="grid">
				{#each tags as tag (tag.identifier)}
					<div on:click={() => addTag(tag)}>
						<Tag {tag} />
					</div>
				{/each}
			</div>
			<p>You can create a new tag <a href="/tags">here</a></p>
		</div>
	</div>
{/if}

<style lang="scss">
	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.6);
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.overlay-content {
		border-radius: 0.5rem;
		border-color: var(--text-color);
		border-width: 1px;
		border-style: solid;
		background-color: var(--input-bg);
		padding: 2rem 3rem;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		justify-items: center;
		> * {
			cursor: pointer;
		}
	}

	button {
		background-color: var(--input-bg);
		border: none;
		color: var(--text-color);
		border-radius: 0.5rem;
		padding: 0.75rem 1rem;
		cursor: pointer;
	}
</style>
