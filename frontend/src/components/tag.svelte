<script lang="ts">
	import type { Tag } from '$lib/models/bindings/Tag';
	import { afterUpdate } from 'svelte';

	export let tag: Tag;
	export let close_callback: ((t: Tag) => Promise<any>) | undefined = undefined;

	let elem: HTMLElement;
	afterUpdate(() => {
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

<div class="tagdiv">
	<div bind:this={elem}>
		{tag.name}
	</div>
	{#if close_callback}
		<button
			type="button"
			class="closebutton"
			on:click={(_) => {
				if (close_callback) {
					close_callback(tag);
				}
			}}
		>
			X
		</button>
	{/if}
</div>

<style lang="scss">
	.tagdiv {
		display: grid;
		grid-template-columns: 1fr;
		align-items: center;

		div {
			width: fit-content;
			height: fit-content;
			padding: 0.5rem 0.75rem;
			border-radius: 0.5rem;
		}

		.closebutton {
			justify-self: end;
			color: var(--text-color);
			border: none;
			position: absolute;
			cursor: pointer;
			display: none;
			background-color: var(--input-bg);
			border-radius: 0.3rem;
			padding: 0.25rem;
			margin: 0;
			margin-right: 0.2rem;
		}
		&:hover .closebutton {
			display: block;
		}
	}
</style>
