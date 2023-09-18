<script lang="ts">
	import type { Control } from '$lib/models/bindings/Control';
	import type { Measure } from '$lib/models/bindings/Measure';
	import { afterUpdate } from 'svelte';
	import { default as ForceGraph3D, type ForceGraph3DInstance } from '3d-force-graph';
	let elem: Element;

	export let controls: Control[];
	export let measures: Measure[];

	type T = {
		id: string;
		group: 'measure' | 'control';
		title: string;
	};

	export let controls_for_measure: Map<string, [String, Number][]>;
	type LinkObject = object & {
		source?: string;
		target?: string;
		index?: number;
	};

	let links: LinkObject[] = [];
	let c2: Map<string, number> = new Map();

	controls_for_measure.forEach((ctrls, measure_id) => {
		if (measures.find((m) => m.identifier == measure_id)) {
			ctrls.forEach((cc) => {
				const control_id = cc[0].valueOf();
				if (controls.find((c) => c.identifier == control_id)) {
					links.push({
						source: measure_id,
						target: control_id
					});
					c2.set(measure_id + control_id, cc[1].valueOf());
				}
			});
		}
	});

	let nodes: T[] = controls.map((c) => {
		return { id: c.identifier, title: c.title, group: 'control' };
	});
	nodes = nodes.concat(
		measures.map((c) => {
			return { id: c.identifier, title: c.title, group: 'measure' };
		})
	);

	let g: ForceGraph3DInstance;
	let weighted_links = false;

	function init_if_needed() {
		g ??= ForceGraph3D({ rendererConfig: { cavnas: { width: 1000 } } })(elem as HTMLElement)
			.graphData({
				nodes,
				links: links
			})
			.nodeAutoColorBy('group')
			.nodeLabel((node: any) => `<span class="nodelabel">${node.title}</span>`)
			.onNodeClick((node: object) => {
				const n = <T>node;
				if (n.group == 'control') {
					window.open('/controls/' + n.id);
				} else if (n.group == 'measure') {
					window.open('/measures/' + n.id);
				}
			});
	}

	afterUpdate(() => {
		init_if_needed();

		var cssVariableValue = getComputedStyle(document.documentElement).getPropertyValue(
			'--graph-bg'
		);
		g.backgroundColor(cssVariableValue);

		if (weighted_links) {
			g.linkWidth((link) => {
				const key = (link.source as any).id + (link.target as any).id;
				const coverage = c2.get(key) ?? 0;
				return coverage / 40;
			});
		} else {
			g.linkWidth((_) => 1);
		}
	});
</script>

<div aria-label="Graph showing how controls and measures relate to one-another" bind:this={elem} />
<div class="controls">
	<div class="row">
		<input type="checkbox" class="toggle" bind:checked={weighted_links} />
		Weight links by coverage
	</div>
	<button
		on:click={(_) => {
			g?.zoomToFit();
		}}>Zoom to fit</button
	>
</div>

<style lang="scss">
	.controls {
		position: absolute;
		top: 0;
		right: 0;
		padding: 1rem;
		background-color: var(--bg2);
		border-radius: 0.5rem;
		margin: 0.25rem;
	}
	.row {
		gap: 1rem;
	}
	button {
		margin-top: 1em;
		padding: 1em 2em;
		border-radius: 1em;
		border: none;
		color: var(--text-color);
		background-color: var(--input-bg);
		cursor: pointer;
	}
	:global(span.nodelabel) {
		color: var(--text-color);
	}
</style>
