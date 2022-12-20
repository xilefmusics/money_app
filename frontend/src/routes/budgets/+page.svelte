<script>
	import { page } from '$app/stores';
	import List from '../../lib/components/List.svelte';
	import Chart from '../../lib/components/Chart.svelte';

	export let data;
	const budgets = data.budgets;
	const inbudgets = data.inbudgets;
</script>

<div class="main">
	<h1>Quickselector</h1>
	<ul>
		<li><a href={$page.url.pathname}>All Time</a></li>
		<li><a href="{$page.url.pathname}?startDate=2021&endDate=2022">2021</a></li>
		<li><a href="{$page.url.pathname}?startDate=2022&endDate=2023">2022</a></li>
		<li><a href="{$page.url.pathname}?startDate=2022-10&endDate=2022-11">2022 Oktober</a></li>
		<li><a href="{$page.url.pathname}?startDate=2022-11&endDate=2022-12">2022 November</a></li>
		<li><a href="{$page.url.pathname}?startDate=2022-12&endDate=2023">2022 Dezember</a></li>
	</ul>
	<h1>Out</h1>
	<div class="chart-wrapper">
		<Chart
			type="doughnut"
			data={{
				labels: budgets.map((inbudget) => inbudget.name),
				datasets: [
					{
						label: 'Sum',
						data: budgets.map((inbudget) => inbudget.amount / 100),
						borderWidth: 1
					}
				],
				options: {}
			}}
		/>
	</div>
	<List
		items={budgets.map((budget) => ({
			title: budget.name,
			subtitle: null,
			subtitleIcon: null,
			amount: budget.amount,
			color: budget.amount < 0 ? 'green' : budget.amount > 0 ? 'red' : 'gray',
			link: null,
			link2: `/transactions?budget=${budget.name}`,
			link3: null,
			newBlock: null
		}))}
	/>
	<h1>In</h1>
	<div class="chart-wrapper">
		<Chart
			type="doughnut"
			data={{
				labels: inbudgets.map((inbudget) => inbudget.name),
				datasets: [
					{
						label: 'Sum',
						data: inbudgets.map((inbudget) => inbudget.amount / 100),
						borderWidth: 1
					}
				],
				options: {}
			}}
		/>
	</div>
	<List
		items={inbudgets.map((inbudget) => ({
			title: inbudget.name,
			subtitle: null,
			subtitleIcon: null,
			amount: inbudget.amount,
			color: inbudget.amount > 0 ? 'green' : inbudget.amount < 0 ? 'red' : 'gray',
			link: null,
			link2: `/transactions?inbudget=${inbudget.name}`,
			link3: null,
			newBlock: null
		}))}
	/>
</div>

<style>
	.chart-wrapper {
		width: 100%;
		max-width: 40rem;
		text-align: center;
		margin: auto;
	}
</style>
