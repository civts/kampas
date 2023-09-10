<script lang="ts">
	import { afterUpdate } from 'svelte';
	import { select, arc } from 'd3';

	export let value: number;
	export let percent: boolean = false;
	export let max_value = 100;
	export let width = 150;
	export let height = 150;
	export let thickness = 6;
	export let color: string | undefined = undefined;
	export let text: string = `${Math.round(value)}${percent ? '%' : ''}`;

	let svgElement: Element;

	afterUpdate(() => {
		const radius = Math.min(width, height) / 2;

		const svg = select(svgElement).select('g');

		const backgroundArc = arc()
			.innerRadius(radius - thickness)
			.outerRadius(radius)
			.startAngle(0)
			.endAngle(2 * Math.PI);

		svg.select('.background-arc').attr('d', backgroundArc as any);

		const angle = (value / max_value) * (2 * Math.PI);
		const progressArc = arc()
			.innerRadius(radius - thickness)
			.outerRadius(radius)
			.startAngle(0)
			.endAngle(angle);

		svg.select('.progress-arc').attr('d', progressArc as any);
		if (color != undefined) {
			svg.select('.progress-arc').style('fill', color);
		}

		svg.select('.progress-text').text(text);
		text = `${Math.round(value)}${percent ? '%' : ''}`;
	});
</script>

<div class="gauge">
	<svg width="200" height="200" bind:this={svgElement}>
		<g transform="translate(100, 100)">
			<path
				class="background-arc s-KOyT84NVzzT1 s-KOyT84NVzzT1"
				d="M0,-75A75,75,0,1,1,0,75A75,75,0,1,1,0,-75M0,-71A71,71,0,1,0,0,71A71,71,0,1,0,0,-71Z"
			/>
			<path class="progress-arc" />
			<text class="progress-text" text-anchor="middle" dominant-baseline="middle">
				{text}
			</text>
		</g>
	</svg>
</div>

<style>
	.background-arc {
		fill: var(--input-bg);
	}

	.progress-arc {
		fill: var(--accent);
	}

	.progress-text {
		font-weight: bold;
		fill: var(--text-color);
	}
</style>
