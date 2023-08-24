<script lang="ts">
	import { BACKEND_URL } from '$lib/costants';
	import type { PageData, ActionData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<head>
	<title>Kartik frontend</title>
</head>

<section>
	<h1>Add a new control</h1>
	<form action="?/add" method="post">
		<input type="text" name="title" id="newControl" placeholder="Title" />
		<input type="text" name="description" placeholder="Description" />
		<button type="submit">Add</button>
	</form>
	{#if form?.success}
		<p>Successfully added the control!</p>
	{/if}
	{#if form?.reason}
		<p class="error">Failed: {form?.reason}</p>
	{/if}

	<h1>Import controls from file</h1>
	<form action="{BACKEND_URL}/api/v1/controls/upload" method="post" enctype="multipart/form-data">
		<input name="file" type="file" accept="text/csv" />
		<button type="submit">Upload</button>
	</form>
</section>

<section>
	<p>Controls to implement:</p>
	<ol>
		{#each data?.controls || [] as control}
			<li>
				<h3>{control.title}</h3>
				<p>{control.description}</p>
				<a href="/controls/{control.identifier}"> Details </a>
			</li>
		{:else}
			<!-- this block renders when controls.length === 0 -->
			<p>loading...</p>
		{/each}
	</ol>
</section>
