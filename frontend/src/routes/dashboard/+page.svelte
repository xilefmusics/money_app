<script>
	import Chart from "../../lib/components/Chart.svelte";
	import { onMount } from "svelte";

	const isMobile = () => innerHeight > innerWidth;

	let wealthHistory = null;
	let wealthHistory2 = null;
	const reload = async () => {
		wealthHistory = (
			await (
				await fetch(
					`/api/history/wealth?len=${isMobile() ? 6 : 26}&month=1&year=0`,
				)
			).json()
		).map((item) => {
			item.date = `${new Date(item.date).getMonth() + 1}-${new Date(
				item.date,
			).getFullYear()}`;
			return item;
		});
		wealthHistory2 = (
			await (await fetch(`/api/history/wealth?len=4&year=1`)).json()
		).map((item) => {
			item.date = `${new Date(item.date).getFullYear()}`;
			return item;
		});
	};
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
					label: "Sum",
					data: wealthHistory.map((item) => item.sum.value / 100),
					borderWidth: 1,
					borderColor: "blue",
					backgroundColor: "blue",
				},
				{
					label: "Real",
					data: wealthHistory.map((item) => item.real.value / 100),
					borderWidth: 1,
					borderColor: "lightgreen",
					backgroundColor: "lightgreen",
				},
				{
					label: "Debt",
					data: wealthHistory.map((item) => item.debt.value / 100),
					borderWidth: 1,
					borderColor: "purple",
					backgroundColor: "purple",
				},
				{
					label: "In",
					data: wealthHistory.map((item) => item.in.value / 100),
					borderWidth: 1,
					borderColor: "green",
					backgroundColor: "green",
				},
				{
					label: "Out",
					data: wealthHistory.map((item) => item.out.value / 100),
					borderWidth: 1,
					borderColor: "red",
					backgroundColor: "red",
				},
			],
			options: {},
		}}
	/>
{/if}
<h1>Year Balance</h1>
{#if wealthHistory2}
	<div class="year-wrapper">
		{#each wealthHistory2.reverse().slice(1) as historyItem}
			<h2>{historyItem.date}</h2>
			<Chart
				type="doughnut"
				data={{
					labels: [
						"Spending",
						historyItem.real.diff > 0 ? "Saving" : "Overspending",
					],
					datasets: [
						{
							backgroundColor: [
								"red",
								historyItem.real.diff > 0 ? "green" : "darkred",
							],
							data: [
								historyItem.real.diff > 0
									? (historyItem.in.value - historyItem.real.diff) / 100
									: historyItem.in.value / 100,
								historyItem.real.diff > 0
									? historyItem.real.diff / 100
									: -historyItem.real.diff / 100,
							],
							borderWidth: 1,
						},
					],
					options: {},
				}}
			/>
		{/each}
	</div>
{/if}

<style>
	.year-wrapper {
		display: flex;
		flex-wrap: wrap;
	}
</style>
