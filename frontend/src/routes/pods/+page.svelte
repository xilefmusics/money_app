<script>
	import Chart from '../../lib/components/Chart.svelte';
	import List from '../../lib/components/List.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch';

	let pods = null;
	let podHistory = null;
	let datasets = null;
	onMount(async () => {
		podHistory = await (await fetch(`/api/history/pod?len=10&month=3&year=0`)).json();
		pods = Object.keys(podHistory[podHistory.length - 1])
			.filter((key) => key !== 'date')
			.map((key) => ({
				name: key,
				amount: podHistory[podHistory.length - 1][key].value
			}));
		datasets = pods
			.map((item) => item.name)
			.map((name) => ({
				label: name,
				data: podHistory.map((item) => item[name].value / 100),
				borderWidth: 1
			}));
	});
</script>

{#if pods}
	<Chart
		type="line"
		data={{
			labels: podHistory.map(
				(item) => `${new Date(item.date).getMonth() + 1}-${new Date(item.date).getFullYear()}`
			),
			datasets: datasets
		}}
	/>
	<List
		items={pods.map((pod) => ({
			title: pod.name,
			subtitle: null,
			subtitleIcon: null,
			amount: pod.amount,
			color: pod.amount > 0 ? 'green' : pod.amount < 0 ? 'red' : 'gray',
			link: null,
			link2: `/transactions?pod=${pod.name}`,
			link3: null,
			newBlock: null
		}))}
	/>
{/if}

<style>
</style>
