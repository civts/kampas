<script lang="ts">
	import Tag from '../../../components/tag.svelte';
	import type { PageData, ActionData } from './$types';
	import { Color, ColorInput } from 'color-picker-svelte';
	import { enhance } from '$app/forms';

	export let data: PageData;
	export let form: ActionData;
	let color = new Color('#ff3d91');

	let color_hex = '';

	$: color_hex = color.toHexString();
</script>

<head>
	<title>Tags</title>
</head>

<section>
	<h1>Tags</h1>
	{#if data?.tags}
		<ul>
			{#each data.tags as tag}
				<li>
					<Tag {tag} />
				</li>
			{/each}
		</ul>
	{:else}
		<!-- this block renders when controls.length === 0 -->
		<p>loading...</p>
	{/if}
</section>

<section>
	<h1>Add a new tag</h1>
	<form action="?/add_tag" method="post" use:enhance>
		<label for="name">Name</label>
		<input type="text" name="name" placeholder="Name" />
		<input hidden bind:value={color_hex} type="text" name="color_hex" placeholder="Color" />
		<div class="picker-container">
			<ColorInput showAlphaSlider={false} bind:color title="Choose the color" />
		</div>
		<button type="submit">Add</button>
	</form>
	{#if form?.success}
		<p>Successfully added the tag!</p>
	{/if}
	{#if form?.reason}
		<p class="error">Failed: {form?.reason}</p>
	{/if}
</section>

<style lang="scss">
	.picker-container {
		--picker-background: var(--input-bg);
	}

	li {
		padding: 2rem;
	}

	ul {
		display: flex;
		list-style: none;
		flex-wrap: wrap;
	}
</style>
