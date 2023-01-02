<script>
	import List from '../../lib/components/List.svelte';
	import Chart from '../../lib/components/Chart.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch';

	let budgets = null;
	let inbudgets = null;
	onMount(async () => {
		const budgetsResult = (
			await (await fetch(`/api/history/budget?len=1&month=0&year=1`)).json()
		).map((item) => {
			item.date = `${new Date(item.date).getMonth() + 1}-${new Date(item.date).getFullYear()}`;
			return item;
		})[0];
		budgets = Object.keys(budgetsResult)
			.filter((name) => name !== 'date')
			.map((name) => ({
				name: name,
				value: budgetsResult[name].value,
				diff: budgetsResult[name].diff
			}));
		const inbudgetsResult = (
			await (await fetch(`/api/history/inbudget?len=1&month=0&year=1`)).json()
		).map((item) => {
			item.date = `${new Date(item.date).getMonth() + 1}-${new Date(item.date).getFullYear()}`;
			return item;
		})[0];
		inbudgets = Object.keys(inbudgetsResult)
			.filter((name) => name !== 'date')
			.map((name) => ({
				name: name,
				value: inbudgetsResult[name].value,
				diff: inbudgetsResult[name].diff
			}));
	});
</script>

<div class="main">
	<h1>Out</h1>
	{#if budgets}
		<div class="chart-wrapper">
			<Chart
				type="doughnut"
				data={{
					labels: budgets.map((item) => item.name),
					datasets: [
						{
							label: 'Sum',
							data: budgets.map((item) => item.value / 100),
							borderWidth: 1
						}
					],
					options: {}
				}}
			/>
		</div>
		<List
			items={budgets.map((item) => ({
				title: item.name,
				subtitle: null,
				subtitleIcon: null,
				amount: item.value,
				color: item.value < 0 ? 'green' : item.value > 0 ? 'red' : 'gray',
				link: null,
				link2: `/transactions?budget=${item.name}`,
				link3: null,
				newBlock: null
			}))}
		/>
	{/if}
	<h1>In</h1>
	{#if inbudgets}
		<div class="chart-wrapper">
			<Chart
				type="doughnut"
				data={{
					labels: inbudgets.map((item) => item.name),
					datasets: [
						{
							label: 'Sum',
							data: inbudgets.map((item) => item.value / 100),
							borderWidth: 1
						}
					],
					options: {}
				}}
			/>
		</div>
		<List
			items={inbudgets.map((item) => ({
				title: item.name,
				subtitle: null,
				subtitleIcon: null,
				amount: item.value,
				color: item.value > 0 ? 'green' : item.value < 0 ? 'red' : 'gray',
				link: null,
				link2: `/transactions?inbudget=${item.name}`,
				link3: null,
				newBlock: null
			}))}
		/>
	{/if}
</div>

<style>
	.chart-wrapper {
		width: 100%;
		max-width: 40rem;
		text-align: center;
		margin: auto;
	}
</style>
