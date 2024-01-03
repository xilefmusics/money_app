<script>
	import Chart from 'chart.js/auto';
	import { onMount } from 'svelte';

	export let type;
	export let data;
	export let options;

	let chartElement;
	onMount(() => {
		new Chart(chartElement, {
			type: type,
			data: data,
			options: options
		});
	});
</script>

<div id="main">
	<canvas bind:this={chartElement} />
</div>

<style>
</style>
