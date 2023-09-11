<script lang="ts">
	import Graph from '../../../components/graph.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<Graph controls={data.controls} measures={data.measures} controls_for_measure={data.links} />
