<script lang="ts">
	import ControlListItem from '../../../components/control_list_item.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Controls</title>
</head>

<section>
	<p>Controls to implement:</p>
	<ol>
		{#each data?.controls || [] as control}
			<li>
				<ControlListItem {control} />
			</li>
		{:else}
			<p>loading...</p>
		{/each}
	</ol>
</section>
