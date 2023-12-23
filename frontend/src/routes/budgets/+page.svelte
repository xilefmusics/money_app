<script>
	import List from "../../lib/components/List.svelte";
	import Chart from "../../lib/components/Chart.svelte";
	import { onMount } from "svelte";
	import { page } from "$app/stores";
	import url2params from "../../lib/url/url2params";
	import params2url from "../../lib/url/params2url";

	const getBudgets = async (currentYear, year, currentMonth, month, type) => {
		let req_len = currentYear - year + 2;
		let req_year = 1;
		let req_month = 0;
		if (month !== "*") {
			req_len = (currentYear - year) * 12 + (currentMonth - month) + 2;
			req_year = 0;
			req_month = 1;
		}

		const historyItem = (
			await (
				await fetch(
					`/api/history/${type}?len=${req_len}&year=${req_year}&month=${req_month}`,
				)
			).json()
		)[0];

		return Object.entries(historyItem.data)
			.map(([key, value]) => ({
				name: key,
				value: value.diff,
			}))
			.filter((item) => item.value > 0)
			.sort((a, b) => {
				if (a.name < b.name) return -1;
				if (a.name > b.name) return 1;
				return 0;
			});
	};

	let budgets = null;
	let inbudgets = null;
	let saveYear = undefined;
	let saveMonth = undefined;
	const reload = async () => {
		const params = url2params($page.url.href);
		const currentYear = new Date().getFullYear();
		const year = params.year ? params.year : currentYear;
		const currentMonth = new Date().getMonth() + 1;
		const month = params.month ? params.month : "*";
		saveYear = params.year ? params.year : undefined;
		saveMonth = params.month ? params.month : undefined;

		budgets = await getBudgets(
			currentYear,
			year,
			currentMonth,
			month,
			"budgets",
		);
		inbudgets = await getBudgets(
			currentYear,
			year,
			currentMonth,
			month,
			"inbudgets",
		);
	};
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
							label: "Sum",
							data: budgets.map((item) => item.value / 100),
							borderWidth: 1,
						},
					],
					options: {},
				}}
			/>
		</div>
		<List
			items={budgets.map((item) => ({
				title: item.name,
				subtitle: null,
				subtitleIcon: null,
				amount: item.value,
				color: item.value < 0 ? "green" : item.value > 0 ? "red" : "gray",
				link: null,
				link2: params2url("/transactions", {
					year: saveYear,
					month: saveMonth,
					budget: item.name,
				}),
				link3: null,
				newBlock: null,
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
							label: "Sum",
							data: inbudgets.map((item) => item.value / 100),
							borderWidth: 1,
						},
					],
					options: {},
				}}
			/>
		</div>
		<List
			items={inbudgets.map((item) => ({
				title: item.name,
				subtitle: null,
				subtitleIcon: null,
				amount: item.value,
				color: item.value > 0 ? "green" : item.value < 0 ? "red" : "gray",
				link: null,
				link2: params2url("/transactions", {
					year: saveYear,
					month: saveMonth,
					inbudget: item.name,
				}),
				link3: null,
				newBlock: null,
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
