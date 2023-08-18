<script lang="ts">
	import { BACKEND_URL } from '$lib/costants';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Kartik frontend</title>
</head>
<p>Controls to implement:</p>
<ol>
	{#each data?.controls || [] as control}
		<li>
			<h3>{control.title}</h3>
			<p>{control.description}</p>
		</li>
	{:else}
		<!-- this block renders when controls.length === 0 -->
		<p>loading...</p>
	{/each}
</ol>

<form action="{BACKEND_URL}/api/v1/controls" method="post">
	<input type="text" name="title" id="newControl" placeholder="Title" />
	<input type="text" name="description" placeholder="Description" />
	<button type="submit">Add</button>
</form>

<form action="{BACKEND_URL}/api/v1/controls/upload" method="post" enctype="multipart/form-data">
	<input name="file" type="file" accept="text/csv" />
	<input type="submit" value="upload" />
</form>
