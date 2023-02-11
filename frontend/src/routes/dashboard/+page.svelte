<script>
	import Chart from '../../lib/components/Chart.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch';

	let wealthHistory = null;
	let wealthHistory2 = null;
	const reload = async () => {
		wealthHistory = (await (await fetch(`/api/history/wealth?len=24&month=1&year=0`)).json()).map(
			(item) => {
				item.date = `${new Date(item.date).getMonth() + 1}-${new Date(item.date).getFullYear()}`;
				return item;
			}
		);
		wealthHistory2 = (await (await fetch(`/api/history/wealth?len=5&month=0&year=1`)).json()).map(
			(item) => {
				item.date = `${new Date(item.date).getFullYear()}`;
				return item;
			}
		);
	}
	onMount(reload);
</script>

<h1>Dashboard</h1>
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
					data: wealthHistory.map((item) => item.in.diff / 100),
					borderWidth: 1,
					borderColor: 'green',
					backgroundColor: 'green'
				},
				{
					label: 'Out',
					data: wealthHistory.map((item) => item.out.diff / 100),
					borderWidth: 1,
					borderColor: 'red',
					backgroundColor: 'red'
				}
			],
			options: {}
		}}
	/>
{/if}
<h1>Year Balance</h1>
{#if wealthHistory2}
	<div class="year-wrapper">
		<h2>{wealthHistory2[3].date}</h2>
		<Chart
			type="doughnut"
			data={{
				labels: ['Spending', 'Saving'],
				datasets: [
					{
						backgroundColor: ['red', 'green'],
						data: [
							(wealthHistory2[3].in.diff - wealthHistory2[3].real.diff) / 100,
							wealthHistory2[3].real.diff / 100
						],
						borderWidth: 1
					}
				],
				options: {}
			}}
		/>
		<h2>{wealthHistory2[2].date}</h2>
		<Chart
			type="doughnut"
			data={{
				labels: ['Spending', 'Saving'],
				datasets: [
					{
						backgroundColor: ['red', 'green'],
						data: [
							(wealthHistory2[2].in.diff - wealthHistory2[2].real.diff) / 100,
							wealthHistory2[2].real.diff / 100
						],
						borderWidth: 1
					}
				],
				options: {}
			}}
		/>
		<h2>{wealthHistory2[1].date}</h2>
		<Chart
			type="doughnut"
			data={{
				labels: ['Spending', 'Saving'],
				datasets: [
					{
						backgroundColor: ['red', 'green'],
						data: [
							(wealthHistory2[1].in.diff - wealthHistory2[1].real.diff) / 100,
							wealthHistory2[1].real.diff / 100
						],
						borderWidth: 1
					}
				],
				options: {}
			}}
		/>
	</div>
{/if}

<style>
	.year-wrapper {
		display: flex;
	}
</style>
