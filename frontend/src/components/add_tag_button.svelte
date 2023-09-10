<script lang="ts">
	import type { Tag as T } from '$lib/models/bindings/Tag';
	import { afterUpdate, onMount } from 'svelte';
	import Tag from './tag.svelte';

	/**
	 * The function to run when a tag is clicked.
	 * It shall return true to indicate that the overlay can be closed.
	 * False to leave it open.
	 */
	export let callback: (t: T) => Promise<boolean>;

	let showOverlay = false;
	let tags: T[] | undefined = undefined;

	async function fetchTags() {
		const response = await fetch('/api/tags');
		tags = await response.json();
		tags?.sort((a, b) => a.name.localeCompare(b.name));
	}

	afterUpdate(() => {
		if (showOverlay) {
			if (tags == undefined) {
				fetchTags();
			}
		} else {
			tags = undefined;
		}
	});
</script>

<button type="button" on:click={() => (showOverlay = true)}>Add tag</button>

{#if showOverlay}
	<div class="overlay" on:click={() => (showOverlay = false)}>
		<div class="overlay-content" on:click={(e) => e.stopPropagation()}>
			<div class="grid">
				{#each tags ?? [] as tag (tag.identifier)}
					<div
						on:click={async () => {
							if (await callback(tag)) {
								showOverlay = false;
							}
						}}
					>
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
		z-index: 10;
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
		margin-top: 0;
	}
</style>
