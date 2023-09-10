<script lang="ts">
	import type { Measure } from '$lib/models/bindings/Measure';
	import { onMount } from 'svelte';

	/**
	 * The function to run when a measure is selected.
	 * It shall return true to indicate that the overlay can be closed.
	 * False to leave it open.
	 */
	export let on_select: (m: Measure, coverage: number) => Promise<boolean>;

	let showOverlay = false;
	let measures: Measure[] = [];

	async function fetchMeasures() {
		const response = await fetch('/api/measures');
		measures = await response.json();
		measures.sort((a, b) => {
			if (a.title.toLowerCase() > b.title.toLowerCase()) {
				return 1;
			}
			return -1;
		});
	}

	onMount(fetchMeasures);
</script>

<button type="button" on:click={() => (showOverlay = true)}>Add measure</button>

{#if showOverlay}
	<div class="overlay" on:click={() => (showOverlay = false)}>
		<div class="overlay-content" on:click={(e) => e.stopPropagation()}>
			<div class="grid">
				{#each measures as measure (measure.identifier)}
					<div
						on:click={async () => {
							let res = prompt('what is the coverage?');
							try {
								if (res == undefined) {
									throw 'whatever';
								}
								let cov = Number.parseInt(res);
								if (cov > 0 && cov <= 100) {
									if (await on_select(measure, cov)) {
										showOverlay = false;
										return;
									}
								}
							} catch (error) {
								alert('Invalid coverage: must be an integer between 1 and 100');
							}
						}}
					>
						{measure.title}
					</div>
				{/each}
			</div>
			<p>You can create a new measure <a href="/new_measure">here</a></p>
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
		position: relative;
		margin: auto;
		max-width: 80%;
		max-height: 80%;
		border-radius: 0.5rem;
		border-color: var(--text-color);
		border-width: 1px;
		border-style: solid;
		background-color: var(--input-bg);
		padding: 2rem 3rem;
		overflow-y: auto;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 1rem;
		justify-items: start;
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
