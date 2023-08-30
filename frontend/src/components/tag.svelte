<script lang="ts">
	import type { Tag } from '$lib/models/bindings/Tag';
	import { afterUpdate, onMount } from 'svelte';

	export let tag: Tag;
	let elem: HTMLElement;
	onMount(() => {
		elem.style.backgroundColor = `#${tag.color_hex}`;
		const textColor = isColorLight(tag.color_hex) ? 'var(--input-bg)' : 'var(--text-color)';
		elem.style.color = textColor;
	});

	function isColorLight(color: String) {
		// Convert color to RGB format
		const rgb = color.match(/\w{2}/g)!.map((c) => parseInt(c, 16));
		const luminance = (0.299 * rgb[0] + 0.587 * rgb[1] + 0.114 * rgb[2]) / 255;

		return luminance > 0.5;
	}
</script>

<div bind:this={elem}>
	{tag.name}
</div>

<style lang="scss">
	div {
		width: fit-content;
		height: fit-content;
		padding: 0.5rem 0.75rem;
		border-radius: 0.5rem;
	}
</style>
