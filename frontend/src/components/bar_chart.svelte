<script lang="ts">
	import { onMount } from 'svelte';
	import { select, scaleLinear, axisBottom, axisLeft, scaleBand } from 'd3';

	export let data: number[];
	export let chartWidth = 500;
	export let chartHeight = 300;

	onMount(() => {
		if (data.length == 0) {
			console.log('Not showing the graph since we have nothing show');
			return;
		}

		let mindata = Math.round(Math.min(...data));
		let maxdata = Math.round(Math.max(...data));

		let frequency = Array(maxdata - mindata + 1).fill(0);
		data.forEach((n) => {
			const idx = Math.round(n - mindata);
			frequency[idx] += 1;
		});
		let maxFreq = Math.max(...frequency);

		const margin = { vertical: 30, horizontal: 40 };

		// set the dimensions and margins of the graph
		const width = chartWidth - margin.horizontal * 2;
		const height = chartHeight - margin.vertical * 2;

		// append the svg object to the body of the page
		let svg = select('.chart')
			.attr('width', width + margin.horizontal * 2)
			.attr('height', height + margin.vertical * 2)
			.append('g')
			.attr('transform', 'translate(' + margin.horizontal + ',' + margin.vertical + ')');

		// X axis
		let x = scaleBand()
			.range([0, width])
			.domain(
				frequency.map(function (_, index) {
					return Math.round(index).toString();
				})
			)
			.padding(0.2);
		svg
			.append('g')
			.attr('transform', 'translate(0,' + height + ')')
			.call(axisBottom(x))
			.selectAll('text')
			.attr('transform', 'translate(-10,0)rotate(-45)')
			.style('text-anchor', 'end');

		// Add Y axis
		let y = scaleLinear().domain([0, maxFreq]).range([height, 0]);
		svg.append('g').call(axisLeft(y));

		// Data bars
		svg
			.selectAll('graphbar')
			.data(frequency)
			.enter()
			.append('rect')
			.attr('x', (_freq, index) => {
				return x(Math.round(index).toString())!;
			})
			.attr('y', function (freq) {
				return y(freq);
			})
			.attr('fill', 'var(--accent)')
			.attr('width', x.bandwidth())
			.attr('height', function (freq) {
				return y(0) - y(freq);
			});
	});
</script>

<svg class="chart" />

<style lang="scss">
	svg {
		color: var(--text);
	}
</style>
