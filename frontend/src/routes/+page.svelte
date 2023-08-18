<script lang="ts">
	import { onMount } from 'svelte';
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<head>
	<title>Kartik frontend</title>
</head>

Welcome
{#if data.user}
	{data.user}!
{:else}
	unknown visitor, login <a href="/login">here</a>
{/if}
