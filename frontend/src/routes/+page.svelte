<script lang="ts">
	import Identicon from 'identicon.js';
	import type { PageData } from './$types';

	export let data: PageData;

	const identicon_data = new Identicon(data.user_hash, {
		size: 100,
		format: 'svg'
	}).toString();
</script>

<head>
	<title>Kartik frontend</title>
</head>

Welcome
{#if data.user}
	{data.user}!
	<img
		width="50"
		height="50"
		src="data:image/svg+xml;base64,{identicon_data}"
		alt="{data.user}'s profile picture"
	/>
{:else}
	unknown visitor, login <a href="/login">here</a>
{/if}
