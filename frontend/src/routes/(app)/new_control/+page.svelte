<script lang="ts">
	import { enhance } from '$app/forms';
	import { BACKEND_URL } from '$lib/costants';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<head>
	<title>Add Control</title>
</head>

<section>
	<h1>Add a new control</h1>
	<form action="?/add" method="post">
		<label for="title">Title</label>
		<input type="text" name="title" id="title" placeholder="Title" />
		<label for="description">Description</label>
		<input type="text" name="description" id="description" placeholder="Description" />
		<button type="submit">Add</button>
	</form>
	{#if form?.success}
		<p>Successfully added the control!</p>
	{/if}
	{#if form?.reason}
		<p class="error">Failed: {form?.reason}</p>
	{/if}
</section>

<section>
	<h1>Import controls from file</h1>
	<form
		action="{BACKEND_URL}/api/v1/controls/upload"
		method="post"
		enctype="multipart/form-data"
		use:enhance
	>
		<label for="file">CSV file with the controls</label>
		<input class="drop-zone" name="file" id="file" type="file" accept="text/csv" />
		<button type="submit">Upload</button>
	</form>
</section>

<style>
	.drop-zone {
		width: 300px;
		height: 200px;
		border: 2px dashed var(--text-color);
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		cursor: pointer;
	}
</style>
