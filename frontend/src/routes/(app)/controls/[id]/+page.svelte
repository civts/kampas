<script lang="ts">
	import TagControlButton from '../../../../components/tag_control_button.svelte';
	import type { PageData } from './$types';

	export let data: PageData;
	let control = data.control;
</script>

{#if control}
	<h1>{control.title}</h1>
	<p>{control.description}</p>
	<TagControlButton control_id={control.identifier} />
{/if}
