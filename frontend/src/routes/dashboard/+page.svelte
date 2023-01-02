<script>
	import Chart from '../../lib/components/Chart.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch'

	let wealthHistory = null
    onMount(async () => {
		wealthHistory = (await (await fetch(`/api/history/wealth?len=10&month=3&year=0`)).json()).map(item => {
			item.date = `${new Date(item.date).getMonth()+1}-${new Date(item.date).getFullYear()}`;
			return item
		})
    })
</script>

{#if wealthHistory}
<Chart
	type="line"
	data={{
		labels: wealthHistory.map((item) => item.date),
		datasets: [
			{
				label: 'Sum',
				data: wealthHistory.map((item) => item.sum.value / 100),
				borderWidth: 1,
				borderColor: 'blue',
				backgroundColor: 'blue'
			},
			{
				label: 'Real',
				data: wealthHistory.map((item) => item.real.value / 100),
				borderWidth: 1,
				borderColor: 'lightgreen',
				backgroundColor: 'lightgreen'
			},
			{
				label: 'Debt',
				data: wealthHistory.map((item) => item.debt.value / 100),
				borderWidth: 1,
				borderColor: 'purple',
				backgroundColor: 'purple'
			},
			{
				label: 'In',
				data: wealthHistory.map((item) => item.in.value / 100),
				borderWidth: 1,
				borderColor: 'green',
				backgroundColor: 'green'
			},
			{
				label: 'Out',
				data: wealthHistory.map((item) => item.out.value / 100),
				borderWidth: 1,
				borderColor: 'red',
				backgroundColor: 'red'
			}
		],
		options: {}
	}}
/>
{/if}
