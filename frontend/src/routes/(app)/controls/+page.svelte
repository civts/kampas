<script lang="ts">
	import type { PageData, ActionData } from './$types';

	export let data: PageData;

	export let form: ActionData;
</script>

<head>
	<title>Controls</title>
</head>

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
