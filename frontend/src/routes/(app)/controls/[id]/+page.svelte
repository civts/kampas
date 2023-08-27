<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
	let control = data.control;
</script>

{#if control}
	<h1>{control.title}</h1>
	<p>{control.description}</p>
{/if}
