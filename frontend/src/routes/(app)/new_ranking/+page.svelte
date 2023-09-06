<script lang="ts">
	import { enhance } from '$app/forms';
	import { redirect } from '@sveltejs/kit';
	import { afterUpdate } from 'svelte';
	import type { ActionData } from './$types';

	export let form: ActionData;

	afterUpdate(() => {
		console.log('Running againn');
		if (form?.ranking_id) {
			const to = `/rankings/${form.ranking_id}`;
			console.log('Redirecting', to);
			window.location.href = to;
		}
	});
</script>

<head>
	<title>Create Ranking</title>
</head>

<section>
	<h1>Add a new ranking</h1>
	<div class="createform">
		<form action="?/add" method="post" use:enhance>
			<label for="minimum_coverage">Minimum coverage (for each control)</label>
			<input
				type="number"
				min="1"
				max="100"
				name="minimum_coverage"
				value="100"
				placeholder="Mimimum coverage for each control"
			/>
			<button type="submit">Okay</button>
		</form>
		{#if form?.reason}
			<p class="error">Failed: {form?.reason}</p>
		{/if}
	</div>
</section>
