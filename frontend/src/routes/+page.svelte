<script lang="ts">
	import { onMount } from 'svelte';

	type Control = {
		title: String;
		description: String;
	};

	let controls: Control[] = [];

	onMount(async () => {
		const a = await fetch('http://localhost:8000/api/v1/controls');
		controls = await a.json();
	});
</script>

<head>
	<title>Kartik frontend</title>
</head>
<p>Controls to implement:</p>
<ol>
	{#each controls as control}
		<li>
			<h3>{control.title}</h3>
			<p>{control.description}</p>
		</li>
	{:else}
		<!-- this block renders when photos.length === 0 -->
		<p>loading...</p>
	{/each}
</ol>

<form action="http://localhost:8000/api/v1/controls" method="post">
	<input type="text" name="title" id="newControl" placeholder="Title" />
	<input type="text" name="description" placeholder="Description" />
	<button type="submit">Add</button>
</form>

<form
	action="http://localhost:8000/api/v1/controls/upload"
	method="post"
	enctype="multipart/form-data"
>
	<input name="file" type="file" accept="text/csv" />
	<input type="submit" value="upload" />
</form>

<style lang="scss">
	form {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1em;
		input {
			background-color: rgb(5, 11, 15);
			color: white;
			padding: 0.5em;
			border-radius: 0.4em;
			border: none;
		}
		button {
			padding: 1em 2em;
			border-radius: 1em;
			border: none;
			background-color: aquamarine;
			cursor: pointer;
		}
	}
</style>
