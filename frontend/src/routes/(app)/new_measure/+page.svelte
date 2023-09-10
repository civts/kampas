<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	export let form: ActionData;
</script>

<section>
	<h1>Add measure</h1>
	<form action="?/add" method="post" use:enhance>
		{#if form?.title_error}
			<p class="error">{form.title_error}</p>
		{/if}
		<label for="title">Title</label>
		<input type="text" name="title" id="title" placeholder="Title" />
		{#if form?.description_error}
			<p class="error">{form.description_error}</p>
		{/if}
		<label for="description">Description</label>
		<input type="text" name="description" id="description" placeholder="Description" />
		{#if form?.effort_error}
			<p class="error">{form.effort_error}</p>
		{/if}
		<label for="effort">Effort</label>
		<input type="number" min="1" name="effort" id="effort" placeholder="Effort" />

		<button type="submit">Add</button>
		{#if form?.success}
			<p>Measure added successfully</p>
		{/if}
		{#if form?.reason}
			<p class="error">Adding the measure failed: {form.reason}</p>
		{/if}
	</form>
</section>

<style lang="scss">
	form {
		display: flex;
		width: 300px;
		margin: 0rem auto;
		align-items: start;
	}
</style>
