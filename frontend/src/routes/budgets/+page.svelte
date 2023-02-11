<script>
	import List from '../../lib/components/List.svelte';
	import Chart from '../../lib/components/Chart.svelte';
	import { onMount } from 'svelte';
	import fetch from '../../lib/api/fetch';
	import { page } from '$app/stores';
  	import url2params from '../../lib/url/url2params';
	import params2url from '../../lib/url/params2url';


	const getBudgets = async (currentYear, year, currentMonth, month, type) => {
		const dateSelector = `${month==="*"?1:month}-${year==="*"?currentYear+1:year}`

		const budgetsResult = (
			await (await fetch(`/api/history/${type}?len=${year==="*"?1:month==="*"?currentYear-year+2:(currentYear-year+1)*(currentMonth-month+1)+2}&month=${month==="*"?0:1}&year=${month==="*"?1:0}`)).json()
		).map((item) => {
			item.date = `${new Date(item.date).getMonth() + 1}-${new Date(item.date).getFullYear()}`;
			return item;
		})
		.filter(b => b.date === dateSelector)[0]

		return Object.keys(budgetsResult)
			.filter((name) => name !== 'date')
			.map((name) => ({
				name: name,
				value: year==="*" ? budgetsResult[name].value : budgetsResult[name].diff,
			}))
			.filter(budget => budget.value !== 0)
	}

	let budgets = null;
	let inbudgets = null;
	let saveYear = undefined;
	let saveMonth = undefined;
	const reload = async () => {
		const params = url2params($page.url.href)
		const currentYear = (new Date()).getFullYear()
		const year = params.year ? params.year : currentYear
		const currentMonth = (new Date()).getMonth() + 1
		const month = params.month ? params.month : "*"
		saveYear = params.year ? params.year : undefined
		saveMonth = params.month ? params.month : undefined

		
		budgets = await getBudgets(currentYear, year, currentMonth, month, "budget")
		inbudgets = await getBudgets(currentYear, year, currentMonth, month, "inbudget")
	}
	onMount(reload);
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
				link2: params2url("/transactions", {year: saveYear, month: saveMonth, budget: item.name}),
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
				link2: params2url("/transactions", {year: saveYear, month: saveMonth, inbudget: item.name}),
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
